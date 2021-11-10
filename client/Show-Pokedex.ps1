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
    $Random,

    [parameter(Mandatory=$false)][int]
    $Scroll,

    [parameter(Mandatory=$false)][int]
    $ScrollTime = 10

)

#PoshMon Tests#
Import-module .\PoshmonGraphicsModule.psm1
Import-module .\PoshdexModule

$poke_e = [char][int]"0x00e9"
$pokedex = Get-Content '../data/pokedex.json' | ConvertFrom-Json
$font_file = Get-Content '../data/font.json' | ConvertFrom-Json
$sprite_atlas = Get-Content '../data/sprite_atlas.json' | ConvertFrom-Json

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

foreach($mon in $pokedex) {
    $mon.front_sprite = Convert-Sprite $mon.front_sprite
    $mon.back_sprite = Convert-Sprite $mon.back_sprite
}

Set-Alphabet -Alphabet $alphabet
Set-SpriteAtlas $sprite_atlas

if($PSBoundParameters.ContainsKey('PokedexIndex')) {
    $target_mon = $pokedex | Where-Object {$_.pokedex -eq $PokedexIndex}
    $pokedex = $pokedex | Sort-Object -Property {$_.pokedex}
    if ($target_mon -eq $null) {
        Write-Error "There is no Pok$($poke_e)mon with Pok$($poke_e)dex of $PokedexIndex"
        exit 1
    }
} elseif($PSBoundParameters.ContainsKey('InternalIndex')) {
    $target_mon = $pokedex | Where-Object {$_.index -eq $InternalIndex}
    $pokedex = $pokedex | Sort-Object -Property {$_.index}
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
    $ScrollRandom = $True
    $target_mon = $pokedex | Get-Random
} elseif($PSBoundParameters.ContainsKey('Scroll')) {
    $Scroll_Check = $True
}

if ($Scroll -lt 0) {
    while(!$check) {
        if ($Host.UI.RawUI.KeyAvailable) {
            $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
            if ($key.keydown -eq "True") {
                $check = $true
            }
        }
        if (!$check) {
            for ($clearint=0;$clearint -lt 7; $clearint++) {
                Write-Text "       " -x 1 -y (1+$clearint) -Tile
            }
            Show-DexEntry $target_mon -NoDisplay:$NoDisplay -Scroll -ScrollTime:$ScrollTime
            if(!$ScrollRandom) {
                $target_mon = $pokedex[(($pokedex.indexof($target_mon) + 1) % $pokedex.Length)]
            } else {
                $target_mon = $pokedex | Get-Random
            }
        }
    }
} elseif ($Scroll -ge 0 -and $Scroll_Check) {
    for($scroll_index = 0; $scroll_index -le $Scroll; $scroll_index++) {
        if ($Host.UI.RawUI.KeyAvailable) {
            $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
            if ($key.keydown -eq "True") {
                $check = $true
                $scroll_index = $Scroll + 1
            }
        }
        if (!$check) {
            for ($clearint=0;$clearint -lt 7; $clearint++) {
                Write-Text "       " -x 1 -y (1+$clearint) -Tile
            }
            Show-DexEntry $target_mon -NoDisplay:$NoDisplay -Scroll -ScrollTime:$ScrollTime
            if(!$ScrollRandom) {
                $target_mon = $pokedex[(($pokedex.indexof($target_mon) + 1) % $pokedex.Length)]
            } else {
                $target_mon = $pokedex | Get-Random
            }
        }
    }
} else {
    Show-DexEntry $target_mon -NoDisplay:$NoDisplay
}


# 

if (!$NoClear) {
    while(!$clear_check) {
        # if ($Host.UI.RawUI.KeyAvailable) {
        #     $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
        #     if ($key.keydown -eq "True") {
                Clear-Host
                $clear_check = $true
            # }
        # }
    }
}

remove-module PoshmonGraphicsModule
remove-module PoshdexModule
