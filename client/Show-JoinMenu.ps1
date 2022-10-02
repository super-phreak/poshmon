param(
    # Debug run starts the main method of the script to run as a stand alone script.
    [Parameter(Mandatory=$false)]
    [Switch]
    $DebugRun,
    [parameter(Mandatory=$false)][Switch]
    $NoDisplay,

    [parameter(Mandatory=$false)][Switch]
    $NoClear,

    [parameter(Mandatory=$false)][Int]
    $SpoofGames
)

function Show-JoinMenu {
    Param(
        [Parameter(Mandatory=$true)]
        [AllowEmptyCollection()]
        [Collections.Generic.List[PSCustomObject]]
        $GameList,
        [Parameter(Mandatory=$true)]
        [Int]$GameTotal
    )

    $CurrentIndex = 0;
    $TopList = 0;
    # if ($null -eq $GameList.Count) {
    #     $currentGames = 0
    # } else {
    $currentGames = $GameList.Count
    # }

    for ($i=$GameList.Count;$i -lt 5;$i++) {
        $GameList.add([PSCustomObject]@{
            name = '----------'
            currentUsers = '-'
            totalUsers = '-'
        })
    }

    Add-VBuff $sprite_atlas.pokedex_tiles.sprite_sheet[17] -X 0 -Y 0 -Tile
    for ($i=1;$i -lt 12;$i++) {
        Add-VBuff -Sprite $sprite_atlas.pokedex_tiles.sprite_sheet[16 + ($i%2)] -X 0 -Y $i -Tile
    }
    Add-VBuff -Sprite $sprite_atlas.pokedex_tiles.sprite_sheet[13] -X 0 -Y 12 -Tile
    Add-VBuff -Sprite $sprite_atlas.pokedex_tiles.sprite_sheet[4] -X 0 -Y 13 -Tile
    for ($i=1; $i -lt 20; $i++) {
        Add-VBuff -Sprite $sprite_atlas.pokedex_tiles.sprite_sheet[4] -X $i -Y 13 -Tile
        Add-VBuff -Sprite $sprite_atlas.pokedex_tiles.sprite_sheet[15] -X $i -Y 12 -Tile
    }
    Add-VBuff $sprite_atlas.pokedex_tiles.sprite_sheet[17] -X 0 -Y ((11*8)+2)

    for ($i=0;$i -lt 5;$i++) {
        Write-Text "$($GameList[$i+$TopList].Name.Substring(0,5)) $($GameList[$i+$TopList].Name.Substring(5,5)) $($GameList[$i].currentUsers)/$($GameList[$i].TotalUsers)" -X 3 -Y (2+$i*2)
    }
    Write-Text ">" -X 2 -Y (2 + ($CurrentIndex*2))   

    if (($CurrentIndex+5) -lt $GameList.Count) {
        Write-Text "_" -X 19 -Y 11
    }
    
    $game_text = "CHOOSE A GAME"
    Write-Text $game_text -X 3 -Y 0

    Write-Text "Total Games: $($currentGames.ToString().PadLeft(2,'0'))/$GameTotal" -X 1 -Y 14
    Write-Text "Join: ----- -----" -X 1 -Y 16
    #Add-VBuff -Sprite $alphabet[">"] -X 2 -Y 16 -Tile

    Write-Screen -NoDisplay:$NoDisplay

    $made_selection = $false
    $join_string = "----------"
    $stringIndex = 0
    while (!$made_selection) {
        if ($Host.UI.RawUI.KeyAvailable) {
            $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
            if ($key.keydown -eq "True") {
                switch ($key.VirtualKeyCode) {
                    08 {          
                        $stringIndex = [System.Math]::Max([System.Math]::Min($stringIndex-1,9),0)              
                        $join_string = $join_string.Remove($stringIndex,1).Insert($stringIndex,'-')
                        Write-Text "$($join_string.Substring(0,5)) $($join_string.Substring(5,5))" -X 7 -Y 16
                        Write-Screen -NoDisplay:$NoDisplay
                    }
                    27 {return 0}
                    13 {
                        if ($join_string -like "*-*") {
                            $join_string = $GameList[$CurrentIndex].Name
                            Write-Text "$($join_string.Substring(0,5)) $($join_string.Substring(5,5))" -X 7 -Y 16
                            $stringIndex = 10
                            Write-Screen -NoDisplay:$NoDisplay
                        } else {
                            return $join_string
                        }
                    }
                    40 {
                        Write-Text " " -X 2 -Y (2 + (($CurrentIndex-$TopList)*2)) 
                        if ($CurrentIndex -lt ($GameList.Count-1)) {
                            $CurrentIndex++
                            if ($CurrentIndex -gt 4 -and ($TopList+5) -lt $GameList.Count) {
                                $TopList++
                            }
                        }
                        Write-Text ">" -X 2 -Y (2 + (($CurrentIndex-$TopList)*2))    
                        for ($i=0;$i -lt 5;$i++) {
                            Write-Text "$($GameList[$i+$TopList].Name.Substring(0,5)) $($GameList[$i+$TopList].Name.Substring(5,5)) $($GameList[$i].currentUsers)/$($GameList[$i].TotalUsers)" -X 3 -Y (2+$i*2)
                        }
                        if (($TopList+5) -lt $GameList.Count) {
                            Write-Text "_" -X 19 -Y 11
                        } else {
                            Write-Text " " -X 19 -Y 11
                        }
                        Write-Screen -NoDisplay:$NoDisplay
                    }
                    38 {
                        Write-Text " " -X 2 -Y (2 + (($CurrentIndex-$TopList)*2)) 
                        if ($CurrentIndex -gt 0) {
                            $CurrentIndex--
                            if ($CurrentIndex -lt $TopList) {
                                $TopList--
                            }
                        }
                        Write-Text ">" -X 2 -Y (2 + (($CurrentIndex-$TopList)*2))    
                        for ($i=0;$i -lt 5;$i++) {
                            Write-Text "$($GameList[$i+$TopList].Name.Substring(0,5)) $($GameList[$i+$TopList].Name.Substring(5,5)) $($GameList[$i].currentUsers)/$($GameList[$i].TotalUsers)" -X 3 -Y (2+$i*2)
                        }

                        if (($TopList+5) -lt $GameList.Count) {
                            Write-Text "_" -X 19 -Y 11
                        } else {
                            Write-Text " " -X 19 -Y 11
                        }

                        Write-Screen -NoDisplay:$NoDisplay
                    }
                    {$_ -ge 65 -and $_ -le 90} {
                        if ($stringIndex -lt $join_string.Length) {
                            $join_string = $join_string.Remove($stringIndex,1).Insert($stringIndex, $key.Character.ToString().ToUpper())
                        }
                        $stringIndex = [System.Math]::Max([System.Math]::Min($stringIndex+1,10),0)
                        Write-Text "$($join_string.Substring(0,5)) $($join_string.Substring(5,5))" -X 7 -Y 16
                        Write-Screen -NoDisplay:$NoDisplay
                    }
                }

            }
        }
        
    }

    

}

Import-Module .\PoshmonGraphicsModule.psm1

if ($DebugRun) {
    $gameList = New-Object Collections.Generic.List[PSCustomObject]

    for ($i=0;$i -lt $SpoofGames;$i++){        
        $gameList.Add([PSCustomObject]@{
            name = ((Get-Random -Count 2 -InputObject (Get-Content '../data/gamenames.txt')) -join '').ToUpper()
            currentUsers = Get-Random -Minimum 1 -Maximum 8
            totalUsers = 8
        })
    }

    Show-JoinMenu -GameList $gameList -GameTotal 50 | Out-Null
    Clear-Screen
}