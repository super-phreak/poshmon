param(
    # Parameter help description
    [Parameter(Mandatory=$false)]
    [Switch]
    $DebugRun,
    [parameter(Mandatory=$false)][Switch]
    $NoDisplay,

    [parameter(Mandatory=$false)][Switch]
    $NoClear,

    [parameter(Mandatory=$true)][string]
    $ConnectionString
)

Import-module .\PoshmonGraphicsModule.psm1
Import-Module .\WebCommunicationsModule.psm1

$script:game_state = [hashtable]::Synchronized(@{})

function Exit-Poshmon {
    Stop-Connection
    Get-EventSubscriber | Unregister-Event
    #remove-module PoshmonGraphicsModule
    Remove-Module WebCommunicationsModule
}

function Test-Signature {
    param(
        [Parameter(Mandatory=$True)]
        $Header,
        [Parameter(Mandatory=$True)]
        $Body,
        [Parameter(Mandatory=$True)]
        $Signature
    )
    $HeaderJson = $Header | ConvertTo-Json -Compress
    $header64 = [System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($HeaderJson))

    $BodyJson = $Body | ConvertTo-Json -Compress
    $body64 = [System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($BodyJson))

    $packetString = "$header64.$body64"
    $testSignature = $script:game_state.hmacsha.ComputeHash([Text.Encoding]::UTF8.GetBytes($packetString))
    $testSignature64 = [System.Convert]::ToBase64String($testSignature)

    return $($testSignature64 -eq $Signature)
}

function global:Read-Message {
    param(
        $msg
    )
    $data = $msg | ConvertFrom-Json
    Write-Host $data.signature | Get-Member
    switch ($data.body.cmd) {
        'login' { Update-Player $data }
        'submit_team' { Update-Team $data }
        Default { Write-Host "UNKNOWN CMD" }
    }
}

function Submit-ForSignature {
    param(
        [Parameter(Mandatory=$True)]
        $Header,
        [Parameter(Mandatory=$True)]
        $Body
    )

    $HeaderJson = $Header | ConvertTo-Json -Compress
    $header64 = [System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($HeaderJson))

    $BodyJson = $Body | ConvertTo-Json -Compress
    $body64 = [System.Convert]::ToBase64String([System.Text.Encoding]::UTF8.GetBytes($BodyJson))

    $packetString = "$header64.$body64"
    $Signature = $script:game_state.hmacsha.ComputeHash([Text.Encoding]::UTF8.GetBytes($packetString))
    $signature64 = [System.Convert]::ToBase64String($Signature)

    return $signature64
}

function Send-Message {
    param(
        $msg
    )
    $header = @{
        alg = "HS256"
        typ = "PWT"
        ver = "0.0.1"
        session_id = $script:game_state.session_id
    }

    $body = $msg

    $signature = Submit-ForSignature -Header $header -Body $body
    Write-Host $signature
}

function Update-Player {
    param(
        $data
    )
    $script:game_state.session_id = $data.header.session_id
    $script:game_state.client_id = $data.body.client_id

    $script:game_state.hmacsha = New-Object System.Security.Cryptography.HMACSHA256
    $script:game_state.hmacsha.key = [System.Convert]::FromBase64String(($data.body.pkey))

    $test = Test-Signature -Header $data.header -Body $data.body -Signature $data.signature
    Write-Host $test
}

function Update-Team {
    param (
        $data
    )
    $script:game_state.team = $data.team
}

function Join-Server {
    param (
        # Parameter help description
        [Parameter(Mandatory=$true)]
        [string]
        $playerName,
        [Parameter(Mandatory=$true)]
        [String]
        $pass
    )
    $game_state.PlayerName = $PlayerName
    Start-Connection -ConnectionString $game_state.ConnectionString -Port $game_state.Port
    $msg = @{
        cmd = "login"
        username = $playerName
        password = $pass
    } | ConvertTo-Json
    Send-MessageJson $msg | Out-Null
    $login_text = "Logging in"
    Write-Text -Text $login_text -X 3 -Y 7 -Tile
    Write-Screen -NoDisplay:$NoDisplay
    for ($i = 0; $i -lt 3; $i++) {
        Write-Text -Text '.' -X (3+$login_text.Length+$i) -Y 7 -Tile
        Write-Screen -NoDisplay:$NoDisplay
        Start-Sleep -Seconds 1
    }
    Write-Text -Text "   Connected!" -line -X 0 -Y 7 -Tile
    Write-Screen -NoDisplay:$NoDisplay
    Start-Sleep -Seconds 2
    Clear-Screen
    Write-Screen -NoDisplay:$NoDisplay
}

function Send-Team {
    param (
        $Mon1,$Mon2,$Mon3,$Mon4,$Mon5,$Mon6
    )
    $team_msg = @{
        cmd = "submit_team"
        session_id = $game_state.session_id
        client_id = $game_state.client_id
        name = $game_state.PlayerName
        team = @(@($Mon1,$Mon2,$Mon3,$Mon4,$Mon5,$Mon6) | Where-Object {$_})
    } | ConvertTo-Json

    Send-MessageJson $team_msg
    $login_text = "Submitting"
    Write-Text -Text $login_text -X 3 -Y 7 -Tile
    Write-Screen -NoDisplay:$NoDisplay
    for ($i = 0; $i -lt 3; $i++) {
        Write-Text -Text '.' -X (3+$login_text.Length+$i) -Y 7 -Tile
        Write-Screen -NoDisplay:$NoDisplay
        Start-Sleep -Seconds 1
    }
    Write-Text -Text "   Submitted!" -line -X 0 -Y 7 -Tile
    Write-Screen -NoDisplay:$NoDisplay
    Start-Sleep -Seconds 2
    Clear-Screen
    Write-Screen -NoDisplay:$NoDisplay
    
}

function Send-Move {
    param (
        # Parameter help description
        [Parameter(Mandatory=$true)]
        [int]
        $MoveId
    )

    $msg = @{
        cmd = "send_move"
        session_id = $game_state.session_id
        client_id = $game_state.client_id
        pokemon_guid = $game_state.team[0].guid
        move_id = $MoveId
    } | ConvertTo-Json

    Send-MessageJson $msg
    
}

function Start-Game {
    Write-Host "Logging in"
    Get-EventSubscriber
    Get-Job
    Get-Event -SourceIdentifier *
    #New-Event -SourceIdentifier "NewServerMessage"

}

###########Insert Area#################
Register-EngineEvent -SourceIdentifier "NewServerMessage" -Action {Read-Message $event.messagedata} | Out-Null

if ($ConnectionString) {
    $game_state.ConnectionString = $ConnectionString.Split(':')[0]
    $game_state.Port = $ConnectionString.Split(':')[1]
} else {
    # $ip = "000.000.000.000"
    # $ip_index = 0
    # Write-Text -Text "Server:" -X 7 -Y 3 -Tile
    # Write-Text -Text $ip -X 2 -Y 4 Tile
    # Write-Screen -NoDisplay:$NoDisplay
    # while ($ip_index -lt 15) {
    #     if ($Host.UI.RawUI.KeyAvailable) {
    #         $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
    #         if ($key.keydown -eq "True") {
    #             switch ($key.VirtualKeyCode) {
    #                 default {Write-Host "$($key.Character),$($key.VirtualKeyCode)"; $ip_index++}
    #             }
    #         }
    #     }
    # }
    $ConnectionString = "172.30.142.149:8080"
    $game_state.ConnectionString = $ConnectionString.Split(':')[0]
    $game_state.Port = $ConnectionString.Split(':')[1]
}

Join-Server -PlayerName "ductape" -Pass "password"
# Send-Team -Mon1 6 -Mon2 25
# Send-Move -MoveId 1
# ./Show-Battle.ps1 -DebugRun -PlayerMon $game_state.team[0] -EnemyMonIndex ($game_state.team[1].id) -NoDisplay:$NoDisplay

while (!$quit) {
    if ($Host.UI.RawUI.KeyAvailable) {
        $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
        if ($key.keydown -eq "True") {
            switch ($key.VirtualKeyCode) {
                81 {$quit = $true; break}
            }
        }
    }
}

if (!$NoClear) {
    Clear-Host
}

Exit-Poshmon