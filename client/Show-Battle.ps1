param(
    # Parameter help description
    [Parameter(Mandatory=$false)]
    [Switch]
    $DebugRun,
    [parameter(Mandatory=$false)][Switch]
    $NoDisplay,

    [parameter(Mandatory=$false)][Switch]
    $NoClear,

    [parameter(Mandatory=$true)]
    $PlayerMon,

    [parameter(Mandatory=$true)][int]
    $EnemyMonIndex
)

function Update-HPBar{
    param(
        # Parameter help description
        [Parameter(Mandatory=$True)]
        [int]
        $CurrentHP,

        [Parameter(Mandatory=$True)]
        [int]
        $MaxHP,

        # Parameter help description
        [Parameter(Mandatory=$false)]
        [Switch]
        $Player
    )
    $hp_bar_full = 9
    $hp_bar_empty = 1
    if($player) {
        $x = 12
        $y = 9

        Write-Text -Text ("$($CurrentHP)".PadLeft(3,' ') + "/" + "$($MaxHP)".PadLeft(3,' ')) -X 11 -Y 10 -Tile
    } else {
        $x = 4
        $y = 2
    }

    for ($precentage=0;($precentage/48) -le ($CurrentHP/$MaxHp);$precentage++) {

    }

    for ($i=0;$i -lt [Math]::Floor(($precentage-1)/8);$i++){
        Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_bar_full] -x ($x+$i) -y $y -Tile
    }
    if ($i -lt 6) {
        $minimum_bar = (&{If($CurrentHP -gt 0 -and $precentage -eq 1) {1} Else {0}})
        Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[(($precentage-1)%8)+1+$minimum_bar] -x ($x+$i) -y $y -Tile
        $i++
        for ( ;$i -lt 6;$i++){
            Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_bar_empty] -x ($x+$i) -y $y -Tile
        }
    }
}

function Clear-EnemyMon {
    $empty_sprite = @{
        data = ,0 * (7*7*64)
        height = 7
        width = 7
    }
    Add-VBuff -Sprite $empty_sprite -X 12 -Y 0 -Tile
}

function Add-BattleTemplate {
    $bend_left = 7
    $arrow_left = 2

    $bend_right = 4
    $arrow_right = 8

    $line = 6
    $vert_line = 3
    $vert_line_dot = 0

    # $elipse = 5
    $hp_word1 = 15
    $hp_word2 = 0
    $hp_bar_end = 10

    Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$vert_line_dot] -x 18 -y 9 -Tile
    Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$vert_line] -x 18 -y 10 -Tile
    Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$bend_left] -x 18 -y 11 -Tile
    Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$arrow_left] -x 9 -y 11 -Tile
    for ($i=17;$i -gt 9;$i--){
        Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$line] -x $i -y 11 -Tile
    }

    Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$vert_line] -x 1 -y 2 -Tile
    Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$bend_right] -x 1 -y 3 -Tile
    Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$arrow_right] -x 10 -y 3 -Tile
    for ($i=2;$i -lt 10;$i++){
        Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$line] -x $i -y 3 -Tile
    }

    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_word1] -x 10 -y 9 -Tile
    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_word2] -x 11 -y 9 -Tile

    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_word1] -x 2 -y 2 -Tile
    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_word2] -x 3 -y 2 -Tile

    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_bar_end] -x 10 -y 2 -Tile

    Add-TextBox 0 12 19 17
}

function Add-BattleMenu {
    param(
        [Parameter(Mandatory=$True)]
        [int]
        $Selection
    )
    $selection_pos = @(
        @{
            x=9
            y=14
        },@{
            x=15
            y=14
        },@{
            x=9
            y=16
        },@{
            x=15
            y=16
        }
    )
    Add-TextBox 8 12 19 17
    Write-Text -Text " FIGHT $([char]0x1D18)$([char]0x1D0D)" -X 9 -Y 14 -Tile
    Write-Text -Text " ITEM  RUN" -X 9 -Y 16 -Tile
    Write-Text -Text '>' -X $selection_pos[$Selection].x -Y $selection_pos[$Selection].y -Tile

}

function Format-Name {
    param(
        # Parameter help description
        [Parameter(Mandatory=$true)]
        [String]
        $Name
    )
    if ($Name.Length -lt 3) {
        return "  $($name)"
    } elseif ($Name.Length -lt 5) {
        return " $($name)"
    }
    return $name
}

function Enter-MoveMenu {
    param (
        [Parameter(Mandatory=$True)]
        $MonMoves
    )
    Add-TextBox 4 12 19 17
    Add-TextBox 0 8 10 12
    Write-Text -Text "TYPE/" -X 1 -Y 9 -Tile -Line -LineLength 9
    $move_selection = 0
    for($i=0;$i -lt $MonMoves.Count; $i++) {
        Write-Text -Text " $(($moves | Where-Object {$_.id -eq $MonMoves[$i]}).Name)" -X 5 -Y (13+$i) -Tile -Line -LineLength 14
    }
    for($i; $i -lt 4; $i++) {
        Write-Text -Text " -" -X 5 -Y (13+$i) -Tile -Line -LineLength 14
    }
    Write-Text -Text '>' -X 5 -Y (13+$move_selection)
    $current_move = $moves | Where-Object {$_.id -eq $MonMoves[$move_selection]}

    Write-Text -Text " $(($engine_config.types | Where-Object {$_.id -eq $current_move.type_id}).name)" -X 1 -Y 10 -Line -LineLength 9
    $formatted_pp = "$($current_move.pp)".PadRight(2," ")
    Write-Text -Text "    $formatted_pp/$formatted_pp" -X 1 -Y 11 -Tile
    Write-Screen -NoDisplay:$NoDisplay
    
    $made_selection = $false
    while (!$made_selection) {
        if ($Host.UI.RawUI.KeyAvailable) {
            $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
            if ($key.keydown -eq "True") {
                switch ($key.VirtualKeyCode) {
                    81 {return -1}
                    08 {return  0}
                    32 {return  1}
                    38 {$move_selection = ($move_selection+($MonMoves.Length-1))%$MonMoves.Length; Write-Screen -NoDisplay:$NoDisplay; break}
                    40 {$move_selection = ($move_selection+1) % $MonMoves.Length; Write-Screen -NoDisplay:$NoDisplay; break}
                }
                for($i=0;$i -lt $MonMoves.Count; $i++) {
                    Write-Text -Text " $(($moves | Where-Object {$_.id -eq $MonMoves[$i]}).Name)" -X 5 -Y (13+$i) -Tile -Line -LineLength 14
                }
                $current_move = $moves | Where-Object {$_.id -eq $MonMoves[$move_selection]}

                Write-Text -Text " $(($engine_config.types | Where-Object {$_.id -eq $current_move.type_id}).name)" -X 1 -Y 10 -Line -LineLength 9
                $formatted_pp = "$($current_move.pp)".PadRight(2," ")
                Write-Text -Text "    $formatted_pp/$formatted_pp" -X 1 -Y 11
                Write-Text -Text '>' -X 5 -Y (13+$move_selection)
                Write-Screen -NoDisplay:$NoDisplay
            }
        }
    }
    

}

function Show-Pokemon {
    param(
        # Parameter help description
        [Parameter(Mandatory=$True)]
        $Pokemon,

        [Parameter(Mandatory=$True)]
        $level,

        # Parameter help description
        [Parameter(Mandatory=$false)]
        [Switch]
        $Player   
    )

    if ($Player) {
        $xs = @(10,14,17,1)
        $ys = @(7,8,8,5)
        $sprite = $Pokemon.back_sprite
        $sprite_width_offset = 0
        $sprite_height_offset = 0
    } else {
        $xs = @(1,4,7,12)
        $ys = @(0,1,1,7)
        $sprite = $Pokemon.front_sprite
        $sprite_width_offset = (&{If($pokemon.front_sprite.width -le 6) {1} Else {0}})
        $sprite_height_offset = $Pokemon.front_sprite.height
        Clear-EnemyMon
    }
    $mon_name = Format-Name $pokemon.name
    Write-Text -Text $mon_name -X $xs[0] -Y $ys[0] -Tile -Line -LineLength 10
    Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[1] -x $xs[1] -y $ys[1] -Tile
    Write-Text -Text $level -X ($xs[2]-$level.Length) -Y $ys[2] -Tile
    Add-VBuff -Sprite $sprite -X ($xs[3]+$sprite_width_offset) -Y ($ys[3]-$sprite_height_offset) -Tile

    # Write-Text -Text $enemy_mon_name -x 1 -y 0 -tile -Line -LineLength 10
    # Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$lvl_icon] -x 4 -y 1 -Tile
    # Write-Text -Text $level -X (7-$level.Length) -Y 1 -Tile
    # # $enemy_sprite_offset = (&{If($enemy_mon.front_sprite.width -le 6) {1} Else {0}})
    # Add-VBuff -Sprite $enemy_mon.front_sprite -x (12+$enemy_sprite_offset) -y (7-$enemy_mon.front_sprite.height) -Tile
}

#PoshMon Tests#
Import-module .\PoshmonGraphicsModule.psm1

$script:moves = Get-Content '../data/movedex.json' | ConvertFrom-Json
$script:engine_config = Get-Content '../data/engine.json' | ConvertFrom-Json

function Start-Battle {
    $player_mon = $pokedex | Where-Object {$_.Pokedex -eq $PlayerMon.Id}
    $enemy_mon = $pokedex | Where-Object {$_.Pokedex -eq $EnemyMonIndex}
    $player_moves = $player_mon.learnable_moves | Get-Random -Count 4
    #$player_moves+=($sub.id)


    Add-BattleTemplate
    Add-BattleMenu 0

    $int_level = $PlayerMon.level
    $level = "$(($int_level))".PadRight(2,' ')
    $max_health = $PlayerMon.Hp
    $selection = 0

    Update-HPBar -CurrentHP $max_health -MaxHP $max_health -Player
    Update-HPBar -CurrentHP $max_health -MaxHP $max_health

    Show-Pokemon $player_mon $level -Player
    Show-Pokemon $enemy_mon $level

    Write-Screen -NoDisplay:$NoDisplay

    $quit = $false
    while (!$quit) {
        if ($Host.UI.RawUI.KeyAvailable) {
            $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
            if ($key.keydown -eq "True") {
                switch ($key.VirtualKeyCode) {
                    81 {$quit = $true; break}
                    ##This strange logic is only here because bitwise ops in pwsh is slooooow...
                    38 {$selection = ($selection+2)%4; Add-BattleMenu $selection; Write-Screen -NoDisplay:$NoDisplay; break}
                    40 {$selection = ($selection+2)%4; Add-BattleMenu $selection; Write-Screen -NoDisplay:$NoDisplay; break}
                    37 {$selection+=(&{If($selection%2 -eq 0) {1} Else {-1}}); Add-BattleMenu $selection; Write-Screen -NoDisplay:$NoDisplay; break}
                    39 {$selection+=(&{If($selection%2 -eq 0) {1} Else {-1}}); Add-BattleMenu $selection; Write-Screen -NoDisplay:$NoDisplay; break}
                    ##End Strange logic
                    32 {
                        switch ($selection) {
                            0 {
                                $result = Enter-MoveMenu $player_moves
                                if ($result -lt 0) {
                                    $quit = $True
                                } elseif ($result -eq 0) {
                                    Clear-TextBox 0 8 4 12
                                    Add-BattleTemplate
                                    Update-HPBar -CurrentHP $max_health -MaxHP $max_health -Player
                                    Show-Pokemon $player_mon $level -Player
                                    Clear-TextBox 1 13 4 18
                                    Add-BattleMenu $selection
                                    Write-Screen -NoDisplay:$NoDisplay;
                                } elseif ($result -gt 0) {
                                    $box_text = "other trainer"
                                    Clear-TextBox 0 8 4 12
                                    Add-BattleTemplate
                                    Update-HPBar -CurrentHP $max_health -MaxHP $max_health -Player
                                    Show-Pokemon $player_mon $level -Player
                                    Clear-TextBox 1 13 4 18
                                    Write-Text "Waiting on the" -X 2 -Y 14 -Tile -Line -LineLength 16
                                    Write-Text $box_text -X 2 -Y 16 -Tile -Line -LineLength 16
                                    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[19] -X (2+$box_text.Length) -Y 16 -Tile
                                    Write-Screen -NoDisplay:$NoDisplay;                                    
                                }

                                break
                            }
                            3 {
                                $run_text = "safely"
                                Clear-TextBox 1 13 4 18
                                Add-BattleTemplate
                                Write-Text "You got away" -X 2 -Y 14 -Tile -Line -LineLength 16
                                Write-Text $run_text -X 2 -Y 16 -Tile -Line -LineLength 16
                                Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[19] -X (2+$run_text.Length) -Y 16 -Tile
                                Write-Screen -NoDisplay:$NoDisplay
                                Start-Sleep 5
                                $quit = $true
                            }
                        }
                    }
                    default {Write-Host "$($key.Character),$($key.VirtualKeyCode)"}
                }
            }
        }

        
    }
    Write-Screen -NoDisplay:$NoDisplay
}

if ($DebugRun) {
    Start-Battle
}