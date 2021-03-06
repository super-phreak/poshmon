param(
    # Parameter help description
    [Parameter(Mandatory=$false)]
    [Switch]
    $DebugRun,
    [parameter(Mandatory=$false)][Switch]
    $NoDisplay,

    [parameter(Mandatory=$false)][Switch]
    $NoClear,

    [parameter(Mandatory=$false)][string]
    $ConnectionString
)

Import-module .\PoshmonGraphicsModule.psm1
Import-Module .\WebCommunicationsModule.psm1

$script:game_state = [hashtable]::Synchronized(@{})

function Exit-Poshmon {
    Stop-Connection
    Get-EventSubscriber | Unregister-Event
    remove-module PoshmonGraphicsModule
    Remove-Module WebCommunicationsModule
}

function global:Read-Message {
    param(
        $msg
    )
    $data = $msg | ConvertFrom-Json
    
    switch ($data.cmd) {
        'login' { Update-Player $data }
        'submit_team' {Update-Team $data}
        Default { Write-Host "UNKNOWN CMD"}
    }
}

function Update-Player {
    param(
        $data
    )
    $script:game_state.session_id = $data.session_id
    $script:game_state.client_id = $data.client_id
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
        $PlayerName
    )
    $game_state.PlayerName = $PlayerName
    Start-Connection -ConnectionString $game_state.ConnectionString -Port $game_state.Port
    $msg = @{
        cmd = "login"
    } | ConvertTo-Json
    Send-MessageJson $msg
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
    $ConnectionString = "192.168.001.195:8080"
    $game_state.ConnectionString = $ConnectionString.Split(':')[0]
    $game_state.Port = $ConnectionString.Split(':')[1]
}

Join-Server -PlayerName "Josh"
Send-Team -Mon1 6 -Mon2 25
./Show-Battle.ps1 -DebugRun -PlayerMon $game_state.team[0] -EnemyMonIndex ($game_state.team[1].id) -NoDisplay:$NoDisplay

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

Clear-Host

Exit-Poshmon