$script:recv_queue = New-Object 'System.Collections.Concurrent.ConcurrentQueue[String]'
$script:send_queue = New-Object 'System.Collections.Concurrent.ConcurrentQueue[String]'

$script:serverData = [hashtable]::Synchronized(@{})
# $serverData.ws = $ws
# $serverData.cts = $cts
# $serverData.ct = $ct

$serverData.host = $Host

function Start-Connection {
    param(
        [parameter(Mandatory=$true)]
        $ConnectionString,
        [parameter(Mandatory=$true)]
        $Port
    )

    $script:ws = New-Object Net.WebSockets.ClientWebSocket
    $script:cts = New-Object Threading.CancellationTokenSource
    $script:ct = New-Object Threading.CancellationToken($false)

    $connectTask = $ws.ConnectAsync("ws://${ConnectionString}:${Port}", $cts.Token)
    do { Sleep(1) }
    until ($connectTask.IsCompleted)

    $recv_job = {
        param($ws, $client_id, $recv_queue, $serverData)
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
                $serverData.ws.State -ne [Net.WebSockets.WebSocketState]::Open -or $taskResult.Result.EndOfMessage
            )

            if (-not [string]::IsNullOrEmpty($jsonResult)) {
                $recv_queue.Enqueue($jsonResult)
                $serverData.host.runspace.events.generateevent("NewServerMessage", "Server", $null, $jsonResult)
                #$serverData.host.ui.writeline($jsonResult)
            }
        }
    }

    $send_job = {
        param($ws, $client_id, $send_queue)
        $ct = New-Object Threading.CancellationToken($false)
        $workitem = $null
        while ($ws.State -eq [Net.WebSockets.WebSocketState]::Open){
            if ($send_queue.TryDequeue([ref] $workitem)) {
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

    $script:recv_runspace = [PowerShell]::Create()
    $recv_runspace.AddScript($recv_job).
        AddParameter("ws", $ws).
        AddParameter("client_id", $client_id).
        AddParameter("recv_queue", $recv_queue).
        AddParameter("serverData", $serverData).BeginInvoke() | Out-Null
    
    $script:send_runspace = [PowerShell]::Create()
    $send_runspace.AddScript($send_job).
        AddParameter("ws", $ws).
        AddParameter("client_id", $client_id).
        AddParameter("send_queue", $send_queue).BeginInvoke() | Out-Null
}

function Stop-Connection {
    $closetask = $ws.CloseAsync(
        [System.Net.WebSockets.WebSocketCloseStatus]::Empty,
        "",
        $ct
    )

    do { Sleep(1) }
    until ($closetask.IsCompleted)
    $ws.Dispose()

    $recv_runspace.Stop()
    $recv_runspace.Dispose()

    $send_runspace.Stop()
    $send_runspace.Dispose()
}

function Send-MessageJson {
    param(
        # Parameter help description
        [Parameter(Mandatory=$true)]
        $msg
    )
    $send_queue.Enqueue($msg)
}

function Get-ServerMessage {
    $msg = $null
    $recv_queue.TryDequeue([ref] $msg)
    Write-Output "Processed message: $msg"
}



Export-ModuleMember -Function Start-Connection
Export-ModuleMember -Function Stop-Connection
Export-ModuleMember -Function Send-MessageJson
Export-ModuleMember -Function Get-ServerMessage
