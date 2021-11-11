function Add-PokedexTemplate {
    Write-Text -Text "HT" -x 9 -y 6 -TILE
    Add-VBuff -Sprite $internal_sprite_atlas.pokedex_tiles.sprite_sheet[0] -x 14 -y 6 -TILE
    Add-VBuff -Sprite $internal_sprite_atlas.pokedex_tiles.sprite_sheet[1] -x 17 -y 6 -TILE
    
    Write-Text -Text "WT" -x 9 -y 8 -TILE
    Write-Text -Text "lb" -x 17 -y 8 -TILE

    Add-VBuff -Sprite $internal_sprite_atlas.hpbar_status.sprite_sheet[18] -x 2 -y 8 -TILE
    Write-Text -Text '<DOT>' -x 3 -y 8 -TILE -Control
    Write-Text -Text '<DOT>' -x 15 -y 8 -TILE -Control
    Add-PokedexBorder
}

function Add-PokedexBorder {
    $topLeftCorner = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[3]
    $topRightCorner =$internal_sprite_atlas.pokedex_tiles.sprite_sheet[5]
    $botLeftCorner = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[12]
    $botRightCorner = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[14]

    $topBar = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[4]
    $botBar = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[15]

    $rightBar = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[7]
    $leftBar = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[6]

    $rightBarDash = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[10]
    $leftBarDash = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[8]

    $box = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[9]
    $dash = $internal_sprite_atlas.pokedex_tiles.sprite_sheet[11]

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

function Show-DexEntry {
    param(
        [parameter(Mandatory=$true)]
        $Pokemon,
        [parameter(Mandatory=$false)][Switch]
        $NoDisplay,
        [parameter(Mandatory=$false)][Switch]
        $Scroll,
        [parameter(Mandatory=$false)][int]
        $ScrollTime=10
    )


    Add-PokedexTemplate

    $dex_offset = (&{If($Pokemon.front_sprite.width -eq 5) {1} Else {0}})
    Add-VBuff -Sprite $Pokemon.front_sprite -X (1+$dex_offset) -Y (8-$Pokemon.front_sprite.height) -FLIP -TILE

    Write-Text "$($Pokemon.name)".PadRight(10,' ') -X 9 -Y 2 -Tile
    Write-Text "$($Pokemon.pokedex_entry.species)".PadRight(10,' ') -X 9 -Y 4 -Tile

    $pokedex_num = "$($Pokemon.pokedex)".PadLeft(3,'0')
    $pokemon_height_feet = "$($Pokemon.pokedex_entry.height.feet)".PadLeft(3,' ')
    $pokemon_height_inches = "$($Pokemon.pokedex_entry.height.inches)".PadLeft(2,'0')
    $pokemon_weight_top = "$([int]($Pokemon.pokedex_entry.weight/10))".PadLeft(4,' ')
    $pokemon_weight_bot = "$(($Pokemon.pokedex_entry.weight%10))"

    Write-Text -Text $pokedex_num -x 4 -y 8 -TILE
    Write-Text $pokemon_height_feet  -x 11 -y 6 -Tile
    Write-Text $pokemon_height_inches  -x 15 -y 6 -Tile
    Write-Text $pokemon_weight_top -x 11 -y 8 -Tile
    Write-Text $pokemon_weight_bot -x 16 -y 8 -Tile

    $dex_entry_page = $Pokemon.pokedex_entry.text.split("^")
    $dex_entry_line = $dex_entry_page[0].split("<")

    for ($i=0;$i -lt $dex_entry_line.Length; $i++) {
        Write-Text $dex_entry_line[$i] -x 1 -y (11+($i*2)) -Tile -Line
    }

    Write-Text '_' -x 18 -y 16 -tile
    $arrow = $false
    Write-Screen -NoDisplay:$NoDisplay

    $count = 0
    $targettime = ((Get-Date).ToFileTime()+(Get-FileTime -Milli 1000))
    $target_scrolltime = ((Get-Date).ToFileTime()+(Get-FileTime -Milli (1000 * $ScrollTime)))
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
        if ($Scroll) {
            if ($deltatime -ge $target_scrolltime) {
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
            if ($Host.UI.RawUI.KeyAvailable) {
                $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
                if ($key.keydown -eq "True") {
                    return $true
                }
            }
        } elseif ($Host.UI.RawUI.KeyAvailable) {
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
    if ($Scroll) {
        $target_scrolltime = ((Get-Date).ToFileTime()+(Get-FileTime -Milli (1000 * $ScrollTime)))
        while ($deltatime -lt $target_scrolltime) {
            $deltatime = (Get-Date).ToFileTime()
            if ($Host.UI.RawUI.KeyAvailable) {
                $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
                if ($key.keydown -eq "True") {
                    return $true
                }
            }
        }
    } else {
        $screen_check = $false
        while(!$screen_check) {
            if ($Host.UI.RawUI.KeyAvailable) {
                $key = $host.ui.RawUI.ReadKey("NoEcho,IncludeKeyUp,IncludeKeyDown")
                if ($key.keydown -eq "True") {
                    $screen_check = $true
                }
            }
        }
    }
}


function Set-SpriteAtlas {
    param(
        [parameter(Mandatory=$True)]
        $SpriteAtlas
    )
    $script:internal_sprite_atlas = $SpriteAtlas
}

$script:internal_sprite_atlas = @()


Export-ModuleMember -Function Show-DexEntry
Export-ModuleMember -Function Set-SpriteAtlas

#Temp for testing to be removed later
# Export-ModuleMember -Function Add-Template
# Export-ModuleMember -Function Add-Border