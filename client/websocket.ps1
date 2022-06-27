<#
References:
    - https://gist.github.com/byt3bl33d3r/910b3161d54c2d6a3d5e1050c4e1013e
    - https://docs.microsoft.com/en-us/dotnet/api/system.net.websockets.clientwebsocket?view=netframework-4.5
    - https://github.com/poshbotio/PoshBot/blob/master/PoshBot/Implementations/Slack/SlackConnection.ps1
    - https://www.leeholmes.com/blog/2018/09/05/producer-consumer-parallelism-in-powershell/
#>

$recv_queue = New-Object 'System.Collections.Concurrent.ConcurrentQueue[String]'
$send_queue = New-Object 'System.Collections.Concurrent.ConcurrentQueue[String]'

$ws = New-Object Net.WebSockets.ClientWebSocket
$cts = New-Object Threading.CancellationTokenSource
$ct = New-Object Threading.CancellationToken($false)

Write-Host "enter address:port"
$connString = Read-Host

Write-Output "Connecting..."
$connectTask = $ws.ConnectAsync("ws://${connString}", $cts.Token)
do { Sleep(1) }
until ($connectTask.IsCompleted)
Write-Output "Connected!"

$recv_job = {
    param($ws, $client_id, $recv_queue)

    $buffer = [Net.WebSockets.WebSocket]::CreateClientBuffer(1024,1024)
    $ct = [Threading.CancellationToken]::new($false)
    $taskResult = $null

    while ($ws.State -eq [Net.WebSockets.WebSocketState]::Open) {
        $jsonResult = ""
        do {
            $taskResult = $ws.ReceiveAsync($buffer, $ct)
            while (-not $taskResult.IsCompleted -and $ws.State -eq [Net.WebSockets.WebSocketState]::Open) {
                [Threading.Thread]::Sleep(10)
            }

            $jsonResult += [Text.Encoding]::UTF8.GetString($buffer, 0, $taskResult.Result.Count)
        } until (
            $ws.State -ne [Net.WebSockets.WebSocketState]::Open -or $taskResult.Result.EndOfMessage
        )

        if (-not [string]::IsNullOrEmpty($jsonResult)) {
            #"Received message(s): $jsonResult" | Out-File -FilePath "logs.txt" -Append
            $recv_queue.Enqueue($jsonResult)
        }
   }
 }

 $send_job = {
    param($ws, $client_id, $send_queue)
    Write-Output "test"
    $ct = New-Object Threading.CancellationToken($false)
    $workitem = $null
    while ($ws.State -eq [Net.WebSockets.WebSocketState]::Open){
        if ($send_queue.TryDequeue([ref] $workitem)) {
            Write-Host "Sending message: $workitem"

            [ArraySegment[byte]]$msg = [Text.Encoding]::UTF8.GetBytes($workitem)
            $ws.SendAsync(
                $msg,
                [System.Net.WebSockets.WebSocketMessageType]::Binary,
                $true,
                $ct
            ).GetAwaiter().GetResult()
        }
    }
 }

Write-Output "Starting recv runspace" 
$recv_runspace = [PowerShell]::Create()
$recv_runspace.AddScript($recv_job).
    AddParameter("ws", $ws).
    AddParameter("client_id", $client_id).
    AddParameter("recv_queue", $recv_queue).BeginInvoke() | Out-Null

Write-Output "Starting send runspace"
$send_runspace = [PowerShell]::Create()
$send_runspace.AddScript($send_job).
    AddParameter("ws", $ws).
    AddParameter("client_id", $client_id).
    AddParameter("send_queue", $send_queue).BeginInvoke() | Out-Null

try {
    $hash = @{
        cmd = "login"
    }

    $login_payload = New-Object PSObject -Property $hash
    $json = ConvertTo-Json $login_payload
    $send_queue.Enqueue($json)
    
    do {
        $msg = $null
        while ($recv_queue.TryDequeue([ref] $msg)) {
            Write-Output "Processed message: $msg"
            $msg = $msg | ConvertFrom-Json
            if ($msg.cmd -eq "login") {
                $client_id = $msg.client_id
                $session_id = $msg.session_id
            }

        }
        if ($Host.UI.RawUI.KeyAvailable) {
            $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyDown")
            $team = @{
                cmd = "submit_team"
                session_id = $session_id
                client_id = $client_id
                name = "Josh"
                team = 1,2,3,150,151,85
            } | ConvertTo-Json
            $send_queue.Enqueue($team)
        }
    } until ($ws.State -ne [Net.WebSockets.WebSocketState]::Open -or ($key.VirtualKeyCode -eq 81))
}
finally {
    Write-Output "Closing WS connection"
    $closetask = $ws.CloseAsync(
        [System.Net.WebSockets.WebSocketCloseStatus]::Empty,
        "",
        $ct
    )

    do { Sleep(1) }
    until ($closetask.IsCompleted)
    $ws.Dispose()

    Write-Output "Stopping runspaces"
    $recv_runspace.Stop()
    $recv_runspace.Dispose()

    $send_runspace.Stop()
    $send_runspace.Dispose()
}