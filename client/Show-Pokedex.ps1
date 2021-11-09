param(
    [parameter(Mandatory=$false,ParameterSetName="PokedexIndex")]
    [Int]$PokedexIndex,

    [parameter(Mandatory=$false,ParameterSetName="InternalIndex")]
    [Int]$InternalIndex,

    [parameter(Mandatory=$false,ParameterSetName="Name")]
    [String]$Name,

    [parameter(Mandatory=$false)][Switch]
    $NoDisplay,

    [parameter(Mandatory=$false)][Switch]
    $NoClear,

    [parameter(Mandatory=$false,ParameterSetName="Random")][Switch]
    $Random
)

function Add-Template {
    Add-VBuff -Sprite $alphabet["H"] -x 9 -y 6 -TILE
    Add-VBuff -Sprite $alphabet["T"] -x 10 -y 6 -TILE
    Add-VBuff -Sprite $sprite_atlas.pokedex_tiles.sprite_sheet[0] -x 14 -y 6 -TILE
    Add-VBuff -Sprite $sprite_atlas.pokedex_tiles.sprite_sheet[1] -x 17 -y 6 -TILE
    
    Add-VBuff -Sprite $alphabet["W"] -x 9 -y 8 -TILE
    Add-VBuff -Sprite $alphabet["T"] -x 10 -y 8 -TILE
    Add-VBuff -Sprite $alphabet["l"] -x 17 -y 8 -TILE
    Add-VBuff -Sprite $alphabet["b"] -x 18 -y 8 -TILE

    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[18] -x 2 -y 8 -TILE
    Add-VBuff -Sprite $alphabet['<DOT>'] -x 3 -y 8 -TILE

    Add-Border
}

function Add-Border {
    $topLeftCorner = $sprite_atlas.pokedex_tiles.sprite_sheet[3]
    $topRightCorner =$sprite_atlas.pokedex_tiles.sprite_sheet[5]
    $botLeftCorner = $sprite_atlas.pokedex_tiles.sprite_sheet[12]
    $botRightCorner = $sprite_atlas.pokedex_tiles.sprite_sheet[14]

    $topBar = $sprite_atlas.pokedex_tiles.sprite_sheet[4]
    $botBar = $sprite_atlas.pokedex_tiles.sprite_sheet[15]

    $rightBar = $sprite_atlas.pokedex_tiles.sprite_sheet[7]
    $leftBar = $sprite_atlas.pokedex_tiles.sprite_sheet[6]

    $rightBarDash = $sprite_atlas.pokedex_tiles.sprite_sheet[10]
    $leftBarDash = $sprite_atlas.pokedex_tiles.sprite_sheet[8]

    $box = $sprite_atlas.pokedex_tiles.sprite_sheet[9]
    $dash = $sprite_atlas.pokedex_tiles.sprite_sheet[11]

    #Add in the four corners first
    Add-VBuff -Sprite $topLeftCorner -x 0 -y 0
    Add-VBuff -Sprite $topRightCorner -x 19 -y 0 -TILE
    Add-VBuff -Sprite $botLeftCorner -x 0 -y 17 -TILE
    Add-VBuff -Sprite $botRightCorner -x 19 -y 17 -TILE

    #add top/bot border
    for ($i=1;$i -lt 19;$i++) {
        Add-VBuff -Sprite $topBar -x $i -y 0 -TILE
        Add-VBuff -Sprite $botBar -x $i -y 17 -TILE
    }

    #side borders
    for ($i=1;$i -lt 17;$i++) {
        Add-VBuff -Sprite $leftBar -x 0 -y $i -TILE
        Add-VBuff -Sprite $rightBar -x 19 -y $i -TILE
    }

    #overwrite the dashes
    Add-VBuff -Sprite $leftBarDash -x 0 -y 9 -TILE
    Add-VBuff -Sprite $rightBarDash -x 19 -y 9 -TILE

    Add-VBuff -Sprite $box -x 1 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 2 -y 9 -TILE
    Add-VBuff -Sprite $box -x 3 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 4 -y 9 -TILE
    Add-VBuff -Sprite $box -x 5 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 6 -y 9 -TILE
    Add-VBuff -Sprite $box -x 7 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 8 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 9 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 10 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 11 -y 9 -TILE
    Add-VBuff -Sprite $box -x 12 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 13 -y 9 -TILE
    Add-VBuff -Sprite $box -x 14 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 15 -y 9 -TILE
    Add-VBuff -Sprite $box -x 16 -y 9 -TILE
    Add-VBuff -Sprite $dash -x 17 -y 9 -TILE
    Add-VBuff -Sprite $box -x 18 -y 9 -TILE
}

#PoshMon Tests#
$Script:Logfile = "debug.log"
Set-Content -Path $Logfile -Value ""

Import-module .\PoshmonGraphicsModule\PoshmonGraphicsModule.psm1


$pokedex = Get-Content '../data/pokedex.json' | ConvertFrom-Json
$font_file = Get-Content '../data/font.json' | ConvertFrom-Json
$script:sprite_atlas = Get-Content '../data/sprite_atlas.json' | ConvertFrom-Json

$alphabet = New-Object -TypeName System.Collections.Hashtable

foreach($letter in $font_file) {
    $alphabet.add($letter.char, (Convert-Sprite($letter.sprite)))
}

Set-Alphabet -Alphabet $alphabet

for($i=0;$i -lt $sprite_atlas.pokedex_tiles.sprite_sheet.Length;$i++) {
    $sprite_atlas.pokedex_tiles.sprite_sheet[$i] = Convert-Sprite($sprite_atlas.pokedex_tiles.sprite_sheet[$i])
}

for($i=0;$i -lt $sprite_atlas.hpbar_status.sprite_sheet.Length;$i++) {
    $sprite_atlas.hpbar_status.sprite_sheet[$i] = Convert-Sprite($sprite_atlas.hpbar_status.sprite_sheet[$i])
}

foreach($mon in $pokedex) {
    $mon.front_sprite = Convert-Sprite $mon.front_sprite
    $mon.back_sprite = Convert-Sprite $mon.back_sprite
}

if($PSBoundParameters.ContainsKey('PokedexIndex')) {
    $target_mon = $pokedex | Where-Object {$_.pokedex -eq $PokedexIndex}
    if ($target_mon -eq $null) {
        Write-Error "There is no Pok$($poke_e)mon with Pok$($poke_e)dex of $PokedexIndex"
        exit 1
    }
} elseif($PSBoundParameters.ContainsKey('InternalIndex')) {
    $target_mon = $pokedex | Where-Object {$_.index -eq $InternalIndex}
    if ($target_mon -eq $null) {
        Write-Error "There is no Pok$($poke_e)mon with index of $InternalIndex"
        exit 1
    }
} elseif($PSBoundParameters.ContainsKey('Name')) {
    $target_mon = $pokedex | Where-Object {$_.name -eq $Name}
    if ($target_mon -eq $null) {
        Write-Error "There is no Pok$($poke_e)mon with name of $Name"
        exit 1
    }
} elseif($PSBoundParameters.ContainsKey('Random')) {
    $target_mon = $pokedex | Get-Random
}

Add-Template

$dex_offset = (&{If($target_mon.front_sprite.width -eq 5) {1} Else {0}})
Add-VBuff -Sprite $target_mon.front_sprite -X (1+$dex_offset) -Y (7-$target_mon.front_sprite.width+1) -FLIP -TILE

Write-Text $target_mon.name -X 9 -Y 2 -Tile
Write-Text $target_mon.pokedex_entry.species -X 9 -Y 4 -Tile

$pokedex_num = "$($target_mon.pokedex)".PadLeft(3,'0')
$pokemon_height_feet = "$($target_mon.pokedex_entry.height.feet)".PadLeft(3,' ')
$pokemon_height_inches = "$($target_mon.pokedex_entry.height.inches)".PadLeft(2,'0')
$pokemon_weight_top = "$([int]($target_mon.pokedex_entry.weight/10))".PadLeft(4,' ')
$pokemon_weight_bot = "$(($target_mon.pokedex_entry.weight%10))"

Write-Text -Text $pokedex_num -x 4 -y 8 -TILE
Write-Text $pokemon_height_feet  -x 11 -y 6 -Tile
Write-Text $pokemon_height_inches  -x 15 -y 6 -Tile
Write-Text $pokemon_weight_top -x 11 -y 8 -Tile
Add-VBuff -Sprite $alphabet["<DOT>"] -x 15 -y 8 -Tile
Write-Text $pokemon_weight_bot -x 16 -y 8 -Tile

$dex_entry_page = $target_mon.pokedex_entry.text.split("^")
$dex_entry_line = $dex_entry_page[0].split("<")

for ($i=0;$i -lt $dex_entry_line.Length; $i++) {
    Write-Text $dex_entry_line[$i] -x 1 -y (11+($i*2)) -Tile -Line
}

Write-Text '_' -x 18 -y 16 -tile
$arrow = $false
Write-Screen -NoDisplay:$NoDisplay

$count = 0
$targettime = ((Get-Date).ToFileTime()+(Get-FileTime -Milli 1000))

while ($count -lt ($dex_entry_page.Length-1)) {
    $deltatime = (Get-Date).ToFileTime()
    if ($deltatime -ge $targettime) {
        if (!$dex_end) {
            if ($arrow) {
                Write-Text '_' -x 18 -y 16 -tile
                $arrow = !$arrow
            } else {
                Write-Text ' ' -x 18 -y 16 -tile
                $arrow = !$arrow
            }
        }
        $targettime = ((Get-Date).ToFileTime()+(Get-FileTime -Milli 1000))
        Write-Screen -NoDisplay:$NoDisplay
    }
    if ($Host.UI.RawUI.KeyAvailable) {
        $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
        if ($key.keydown -eq "True") {
            $count+=1
            $dex_end = ($dex_entry_page[$count].indexof('}') -ge 0)
            $dex_entry_line_tmp = $dex_entry_page[$count].replace('}', '.')
            $dex_entry_line = $dex_entry_line_tmp.split('<')
            for ($i=0;$i -lt $dex_entry_line.Length; $i++) {
                Write-Text -Text $dex_entry_line[$i] -x 1 -y (11+($i*2)) -Tile -Line
            }
            Write-Text ' ' -x 18 -y 16 -tile
            Write-Screen -NoDisplay:$NoDisplay
        }
    }
}

if (!$NoClear) {
    while(!$check) {
        if ($Host.UI.RawUI.KeyAvailable) {
            $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
            if ($key.keydown -eq "True") {
                Clear-Host
                $check = $true
            }
        }
    }
}

remove-module PoshmonGraphicsModule
