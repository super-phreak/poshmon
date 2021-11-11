param(
    [parameter(Mandatory=$false)][Switch]
    $NoDisplay,

    [parameter(Mandatory=$false)][Switch]
    $NoClear
)

function Exit-Poshmon {
    remove-module PoshmonGraphicsModule
}
$script:hp_bar_full = 9
$script:hp_bar_empty = 1
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

    if($player) {
        $x = 12
        $y = 9

        Write-Text -Text ("$($CurrentHP)".PadLeft(3,' ') + "/" + "$($MaxHP)".PadLeft(3,' ')) -X 11 -Y 10 -Tile
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
    $hp_bar = 9
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

    for ($i=4;$i -lt 10;$i++){
        Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_bar] -x $i -y 2 -Tile
    }
    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$hp_bar_end] -x 10 -y 2 -Tile
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



#PoshMon Tests#
Import-module .\PoshmonGraphicsModule.psm1

#$poke_e = [char][int]"0x00e9"
$pokedex = Get-Content '../data/pokedex.json' | ConvertFrom-Json
$font_file = Get-Content '../data/font.json' | ConvertFrom-Json
$script:sprite_atlas = Get-Content '../data/sprite_atlas.json' | ConvertFrom-Json

$alphabet = New-Object -TypeName System.Collections.Hashtable

foreach($letter in $font_file) {
    $alphabet.add($letter.char, (Convert-Sprite($letter.sprite)))
}

for($i=0;$i -lt $sprite_atlas.pokedex_tiles.sprite_sheet.Length;$i++) {
    $sprite_atlas.pokedex_tiles.sprite_sheet[$i] = Convert-Sprite($sprite_atlas.pokedex_tiles.sprite_sheet[$i])
}

for($i=0;$i -lt $sprite_atlas.hpbar_status.sprite_sheet.Length;$i++) {
    $sprite_atlas.hpbar_status.sprite_sheet[$i] = Convert-Sprite($sprite_atlas.hpbar_status.sprite_sheet[$i])
}

for($i=0;$i -lt $sprite_atlas.battle_hud.sprite_sheet.Length;$i++) {
    $sprite_atlas.battle_hud.sprite_sheet[$i] = Convert-Sprite($sprite_atlas.battle_hud.sprite_sheet[$i])
}


foreach($mon in $pokedex) {
    $mon.front_sprite = Convert-Sprite $mon.front_sprite
    $mon.back_sprite = Convert-Sprite $mon.back_sprite
}

Set-Alphabet -Alphabet $alphabet
#Set-SpriteAtlas $sprite_atlas

$player_mon = $pokedex | Where-Object {$_.name -eq "charmeleon"}
$enemy_mon = $pokedex | Where-Object {$_.name -eq "Blastoise"}


Add-BattleTemplate

$lvl_icon = 1

$player_mon_back_sprite = Resize-Sprite $player_mon.back_sprite -Scale 2
$player_mon_back_sprite.height--
$player_mon_back_sprite.data = $player_mon_back_sprite.data[0..($player_mon_back_sprite.height*$player_mon_back_sprite.width*64)]
$player_mon_name = Format-Name $player_mon.name

$enemy_mon_name = Format-Name $enemy_mon.name
$max_health = 53

## Need to add short name check later
Write-Text -Text $player_mon_name -X 10 -Y 7 -Tile
Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$lvl_icon] -x 14 -y 8 -Tile
Write-Text -Text "99" -X 15 -Y 8 -Tile
Add-VBuff -Sprite $player_mon_back_sprite -X 1 -Y 5 -Tile

Write-Text -Text $enemy_mon_name -x 1 -y 0 -tile
Add-VBuff -Sprite $sprite_atlas.battle_hud.sprite_sheet[$lvl_icon] -x 4 -y 1 -Tile
Write-Text -Text "99" -X 5 -Y 1 -Tile
$enemy_sprite_offset = (&{If($enemy_mon.front_sprite.width -le 6) {1} Else {0}})
Add-VBuff -Sprite $enemy_mon.front_sprite -x (12+$enemy_sprite_offset) -y (7-$enemy_mon.front_sprite.height) -Tile

for ($health=$max_health; $health -ge 0; $health--) {
    Update-HPBar -CurrentHP $health -MaxHP $max_health -Player
    Write-Screen -NoDisplay:$NoDisplay
    Start-Sleep -Milliseconds 125
}

Write-Screen -NoDisplay:$NoDisplay

for ($j = 0; $j -lt 71; $j++) {
    Write-Host
}



Exit-Poshmon



