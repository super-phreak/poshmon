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
    Start-Sleep -Seconds 3
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
    Write-Text -Text "Server:" -X 7 -Y 3 -Tile
    Write-Screen -NoDisplay:$NoDisplay
    $ConnectionString = "192.168.001.195:8080"
    $game_state.ConnectionString = $ConnectionString.Split(':')[0]
    $game_state.Port = $ConnectionString.Split(':')[1]
}

Join-Server -PlayerName "Josh"

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

Exit-Poshmon