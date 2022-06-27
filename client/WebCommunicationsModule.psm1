$recv_queue = New-Object 'System.Collections.Concurrent.ConcurrentQueue[String]'
$send_queue = New-Object 'System.Collections.Concurrent.ConcurrentQueue[String]'



function Start-Connection {
    param(
        [parameter(Mandatory=$true)]
        $ConnectionString,
        [parameter(Mandatory=$true)]
        $Port
    )

    $ws = New-Object Net.WebSockets.ClientWebSocket
    $cts = New-Object Threading.CancellationTokenSource
    $ct = New-Object Threading.CancellationToken($false)

    $script:KillCommand = $false

    $server_job = {
        param($ws, $cts, $ct, $ConnectionString, $Port, $recv_queue, $send_queue)
        $connectTask = $ws.ConnectAsync("ws://${ConnectionString}:${Port}", $cts.Token)
        do { Sleep(1) }
        until ($connectTask.IsCompleted)
        
        try {
            do {
                $msg = $null
                while ($recv_queue.TryDequeue([ref] $msg)) {
                    $msg = $msg | ConvertFrom-Json
                    if ($msg.cmd -eq "login") {
                        $client_id = $msg.client_id
                        $session_id = $msg.session_id
                    }
        
                }
                if ($Host.UI.RawUI.KeyAvailable) {
                    $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyDown")
                    $team = @{
                        cmd = "submit_team"
                        session_id = $session_id
                        client_id = $client_id
                        name = "Josh"
                        team = 1,2,3,150,151,85
                    } | ConvertTo-Json
                    $send_queue.Enqueue($team)
                }
                Start-Sleep -Milliseconds 1000
                Write-Host "Internal KillCommand is ${KillCommand}"
            } until ($ws.State -ne [Net.WebSockets.WebSocketState]::Open)
        
        } finally {
            $closetask = $ws.CloseAsync(
                [System.Net.WebSockets.WebSocketCloseStatus]::Empty,
                "",
                $ct
            )
        
            do { Sleep(1) }
            until ($closetask.IsCompleted)
            $ws.Dispose()
        }
    }

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
                $recv_queue.Enqueue($jsonResult)
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
    $script:recv_runspace.AddScript($recv_job).
    AddParameter("ws", $ws).
    AddParameter("client_id", $client_id).
    AddParameter("recv_queue", $recv_queue).BeginInvoke() | Out-Null


    $script:send_runspace = [PowerShell]::Create()
    $script:send_runspace.AddScript($send_job).
    AddParameter("ws", $ws).
    AddParameter("client_id", $client_id).
    AddParameter("send_queue", $send_queue).BeginInvoke() | Out-Null

    $script:server_runspace = [PowerShell]::Create()
    $script:server_runspace.AddScript($server_job).
    AddParameter("ws", $ws).
    AddParameter("cts", $cts).
    AddParameter("ct", $ct).
    AddParameter("send_queue", $send_queue).
    AddParameter("recv_queue", $recv_queue).
    AddParameter("ConnectionString", $ConnectionString).
    AddParameter("Port", $Port).BeginInvoke() | Out-Null
}

function Stop-Connection {
    $script:server_runspace.Stop()
    $script:server_runspace.Dispose()
    
    $script:recv_runspace.Stop()
    $script:recv_runspace.Dispose()

    $script:send_runspace.Stop()
    $script:send_runspace.Dispose() 
}



Export-ModuleMember -Function Start-Connection
Export-ModuleMember -Function Stop-Connection
