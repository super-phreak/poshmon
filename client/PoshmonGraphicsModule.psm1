param(
    [parameter(Mandatory=$False)][Switch]
    $DEBUG_MODE
)

function Write-Screen{
    param(
        [parameter(Mandatory=$false)][Switch]
        $NoDisplay
    )
    for ($row=0;$row -lt ($CANVAS_HEIGHT);$row++) {
        for ($col=0;$col -lt ($CANVAS_WIDTH);$col++) {
            $canvas_buffer[$row,$col] = $PIXELS[($v_buff[((($row * 2)    ) * $CANVAS_WIDTH) + $col] * 4) + 
                                                 $v_buff[((($row * 2) + 1) * $CANVAS_WIDTH) + $col]]
        }
    }
    $coords = [System.Management.Automation.Host.Coordinates]::new(0,0)
    if (!$NoDisplay) {
        $Host.UI.RawUI.SetBufferContents($coords,$canvas_buffer)
    }
}


function Add-VBuff{
    param(
        [parameter(Mandatory=$true)]
        $Sprite,
        [parameter(Mandatory=$true)]
        [int]$X,
        [parameter(Mandatory=$true)]
        [int]$Y,
        [parameter(Mandatory=$false)][Switch]
        $Flip,
        [parameter(Mandatory=$false)][Switch]
        $Tile
    )
    $offset = $X + ($CANVAS_WIDTH*$Y)
    if ($Tile){
        $offset *= $TILE_SIDE_RAW
    }

    if ($flip) {
        $FLIP_SIGN = -1
        $FLIP_OFFSET = $sprite.width*$TILE_SIDE_RAW-1
    } else {
        $FLIP_SIGN = 1
        $FLIP_OFFSET = 0
    }

    ##Temp function for now
    ##WHY ARRAY MATH SO HARD FOR ME SOMETIMES
    for ($index=0;$index -lt ($sprite.height*$sprite.width*$TILE_SIZE_RAW);$index++) {
            $v_buff[([MATH]::Floor($index/($sprite.width*$TILE_SIDE_RAW))*$CANVAS_WIDTH)+$offset+(($index%($sprite.width*$TILE_SIDE_RAW))*$FLIP_SIGN)+$FLIP_OFFSET] = $sprite.data[$index]
    }
}

function Add-TextBox {
    param(
        [Parameter(Mandatory=$True)]
        [int]
        $LeftX,

        [Parameter(Mandatory=$True)]
        [int]
        $UpperY,
        [Parameter(Mandatory=$True)]
        [int]
        $RightX,

        [Parameter(Mandatory=$True)]
        [int]
        $LowerY

    )
    $text_box_upper_right = 25
    $text_box_upper_left = 23
    $text_box_lower_right = 28
    $text_box_lower_left = 27
    $text_box_vertical = 26
    $text_box_horizontal = 24

    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$text_box_upper_left] -x $LeftX -y $UpperY -Tile
    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$text_box_upper_right] -x $RightX -y $UpperY -Tile
    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$text_box_lower_left] -x $LeftX -y $LowerY -Tile
    Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$text_box_lower_right] -x $RightX -y $LowerY -Tile

    for ($i=($LeftX+1);$i -lt $RightX; $i++) {
        Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$text_box_horizontal] -x $i -y $UpperY -Tile
        Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$text_box_horizontal] -x $i -y $LowerY -Tile
    }

    for ($i=($UpperY+1);$i -lt $LowerY; $i++) {
        Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$text_box_vertical] -x $LeftX  -y $i -Tile
        Add-VBuff -Sprite $sprite_atlas.hpbar_status.sprite_sheet[$text_box_vertical] -x $RightX -y $i -Tile
    }
}

function Clear-TextBox {
    param(
        [Parameter(Mandatory=$True)]
        [int]
        $X,
        [Parameter(Mandatory=$True)]
        [int]
        $Y,
        [Parameter(Mandatory=$True)]
        [int]
        $Lines,
        [Parameter(Mandatory=$True)]
        [int]
        $Length
    )
    $empty_sprite = @{
        data = ,0 * ($Lines*$Length*64)
        height = $Lines
        width = $Length
    }
    Add-VBuff -Sprite $empty_sprite -X $X -Y $Y -Tile
}

function Clear-Screen {
    param(
        [Parameter(Mandatory=$False)]
        [int]
        $X = 0,
        [Parameter(Mandatory=$False)]
        [int]
        $Y = 0,
        [Parameter(Mandatory=$False)]
        [int]
        $Height = 14,
        [Parameter(Mandatory=$False)]
        [int]
        $Width = 19
    )
    $empty_sprite = @{
        data = ,0 * ($Lines*$Length*64)
        height = $Height
        width = $Width
    }
    Add-VBuff -Sprite $empty_sprite -X $X -Y $Y -Tile
}


function Write-Text{
    param(
        [parameter(Mandatory=$true)]
        $Text,
        [parameter(Mandatory=$true)]
        [int]$X,
        [parameter(Mandatory=$true)]
        [int]$Y,
        [parameter(Mandatory=$false)][Switch]
        $Tile,
        [parameter(Mandatory=$false)][Switch]
        $Line,
        [parameter(Mandatory=$false)]
        $LineLength = 18,
        [parameter(Mandatory=$false)][Switch]
        $Control
    )
    if (!$Control) {
        if ($Line) {
            $print_text = $text.padright($LineLength,' ')
        } else {
            $print_text = $text
        }
        for ($i=0;$i -lt $print_text.Length;$i++){
            Add-VBuff -Sprite $alphabet["$($print_text[$i])"] -x ($X+$i) -y $Y -TILE
        }
    } else {
        Add-VBuff -Sprite $alphabet["$($Text)"] -x $X -y $Y -TILE
    }

}

function Write-ScreenDebug {
    param(
        [parameter(Mandatory=$true)]
        $Text,
        [parameter(Mandatory=$false)][Switch]
        $Line
    )

    $Host.UI.RawUI.CursorPosition = $debug_cursor_line
    Write-Host $text
}

function Get-FileTime {
    param(
        [parameter(Mandatory=$true)]
        [int]$Milli
    )
    return $Milli * 10000
}


function Convert-Sprite{
    param(
        [parameter(Mandatory=$true)]
        $Sprite
    )
    $sprite_raw = New-Object 'int[]'  ($Sprite.height*$Sprite.width*$TILE_SIZE_RAW)
    $sprite_decoded = [System.Convert]::FromBase64String($Sprite.data)
    for ($bytenum=0;$bytenum -lt $sprite_decoded.Length;$bytenum++) {
        for ($div=0;$div -lt 4;$div++) {
            $sprite_raw[($bytenum*4)+$div] = (($sprite_decoded[$bytenum] -shr (6 - ($div*2))) -band 3)
        }
    }
    return @{
        data = $sprite_raw
        height = $Sprite.height
        width = $Sprite.width
    }
}

function Resize-Sprite {
    param(
        [parameter(Mandatory=$true)]
        $Sprite,
        
        [Parameter(Mandatory=$true)]
        [int]
        $Scale
    )
    $sprite_scaled = New-Object 'int[]'  ($Sprite.height*$Sprite.width*$TILE_SIZE_RAW*$Scale*$Scale)
    #$sprite_scaled = ,3 * (($Sprite.height+1)*($Sprite.width+1)*$TILE_SIZE_RAW*$Scale*$Scale)
    for ($pixel=0;$pixel -lt $Sprite.data.Length;$pixel++) {
        for ($scale_factor_row=0;$scale_factor_row -lt $Scale;$scale_factor_row++) {
            for ($scale_factor_col=0;$scale_factor_col -lt $Scale;$scale_factor_col++) {
                $sprite_scaled[(($pixel%($Sprite.width*$TILE_SIDE_RAW))*$Scale)+$scale_factor_col+(((([Math]::Floor($pixel/($Sprite.width*$TILE_SIDE_RAW)))*$Scale)+$scale_factor_row)*($Sprite.width*$TILE_SIDE_RAW*$Scale))] = $sprite.data[$pixel]
            }
        }
    }
    return @{
        data = $sprite_scaled
        height = (($Sprite.height)*$Scale)
        width = (($Sprite.width)*$Scale)
    }
}

$script:TILE_SIZE_RAW = 64
$script:TILE_SIDE_RAW = 8
$script:TILE_HIEGHT_POSH = 4

$script:colors = [enum]::GetValues([System.ConsoleColor])
$script:bufferCellType = [enum]::GetValues([System.Management.Automation.Host.BufferCellType])

$script:half_pixel = [char][int]"0x2584"

$script:CANVAS_WIDTH = 160
$script:CANVAS_HEIGHT = 72

$script:v_buff = New-Object 'int[]' ($CANVAS_WIDTH * $CANVAS_HEIGHT * 2)

$script:gb_size = New-Object System.Management.Automation.Host.Size($CANVAS_WIDTH,$CANVAS_HEIGHT)
$script:internal_alphabet = @()
$script:debug_cursor_line = New-Object System.Management.Automation.Host.Coordinates 160, 0

$script:PIXELS = @(
    #data packet 0x0 [00,00] [wh,wh]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::White]),$($colors[[System.ConsoleColor]::White]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x1 [00,01] [wh,gr]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::Gray]),$($colors[[System.ConsoleColor]::White]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x2 [00,10] [wh,dg]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::DarkGray]),$($colors[[System.ConsoleColor]::White]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x3 [00,11] [wh,bl]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::Black]),$($colors[[System.ConsoleColor]::White]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x4 [01,00] [gr,wh]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::White]),$($colors[[System.ConsoleColor]::Gray]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x5 [01,01] [gr,gr]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::Gray]),$($colors[[System.ConsoleColor]::Gray]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x6 [01,10] [gr,dg]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::DarkGray]),$($colors[[System.ConsoleColor]::Gray]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x7 [01,11] [gr,bl]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::Black]),$($colors[[System.ConsoleColor]::Gray]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x8 [10,00] [dr,wh]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::White]),$($colors[[System.ConsoleColor]::DarkGray]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0x9 [10,01] [dr,gr]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::Gray]),$($colors[[System.ConsoleColor]::DarkGray]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0xA [10,10] [dr,dg]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::DarkGray]),$($colors[[System.ConsoleColor]::DarkGray]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0xB [10,11] [dr,bl]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::Black]),$($colors[[System.ConsoleColor]::DarkGray]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0xC [11,00] [bl,wh]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::White]),$($colors[[System.ConsoleColor]::Black]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0xD [11,01] [bl,gr]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::Gray]),$($colors[[System.ConsoleColor]::Black]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0xE [11,10] [bl,dg]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::DarkGray]),$($colors[[System.ConsoleColor]::Black]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete]),
    #data packet 0xF [11,11] [bl,bl]
    [System.Management.Automation.Host.BufferCell]::new($($half_pixel), $($colors[[System.ConsoleColor]::Black]),$($colors[[System.ConsoleColor]::Black]),$bufferCellType[[System.Management.Automation.Host.BufferCellType]::Complete])
)

$Script:canvas_buffer = New-Object 'System.Management.Automation.Host.BufferCell[,]' ($CANVAS_HEIGHT, $CANVAS_WIDTH)

Clear-Host
$Host.UI.RawUI.BackgroundColor = $($colors[[System.ConsoleColor]::Black])
if ($Host.UI.RawUI.WindowSize.Width -gt $gb_size.Width -or $Host.UI.RawUI.WindowSize.Height -gt $gb_size.Height) {
    $Host.UI.RawUI.WindowSize = $gb_size
    $Host.UI.RawUI.BufferSize = $gb_size
}

$Host.UI.RawUI.BufferSize = $gb_size
$Host.UI.RawUI.WindowSize = $gb_size

Write-Progress -Activity "Unpacking sprites..." -ID 0

$sprite_atlas = Get-Content '../data/sprite_atlas.json' | ConvertFrom-Json
$script:alphabet = New-Object -TypeName System.Collections.Hashtable
$font_file = Get-Content '../data/font.json' | ConvertFrom-Json
$script:pokedex = Get-Content '../data/pokedex.json' | ConvertFrom-Json


for($pokedex_tiles=0;$pokedex_tiles -lt $sprite_atlas.pokedex_tiles.sprite_sheet.Length;$pokedex_tiles++) {
    $sprite_atlas.pokedex_tiles.sprite_sheet[$pokedex_tiles] = Convert-Sprite($sprite_atlas.pokedex_tiles.sprite_sheet[$pokedex_tiles])
    Write-Progress -Activity "Unpacking Pokedex Tiles..." -ID 1 -ParentId 0 -PercentComplete (100*$pokedex_tiles/$sprite_atlas.pokedex_tiles.sprite_sheet.Length)
    Write-Progress -Activity "Unpacking sprites..." -ID 0 -PercentComplete (100*($pokedex_tiles+$hpbar_status+$battle_hud+$letter+$mon)/($sprite_atlas.pokedex_tiles.sprite_sheet.Length+$sprite_atlas.hpbar_status.sprite_sheet.Length+$sprite_atlas.battle_hud.sprite_sheet.Length+$font_file.Length+$pokedex.Length))
}

for($hpbar_status=0;$hpbar_status -lt $sprite_atlas.hpbar_status.sprite_sheet.Length;$hpbar_status++) {
    $sprite_atlas.hpbar_status.sprite_sheet[$hpbar_status] = Convert-Sprite($sprite_atlas.hpbar_status.sprite_sheet[$hpbar_status])
    Write-Progress -Activity "Unpacking HP Bar Tiles..." -ID 1 -ParentId 0 -PercentComplete (100*$hpbar_status/$sprite_atlas.hpbar_status.sprite_sheet.Length)
    Write-Progress -Activity "Unpacking sprites..." -ID 0 -PercentComplete (100*($pokedex_tiles+$hpbar_status+$battle_hud+$letter+$mon)/($sprite_atlas.pokedex_tiles.sprite_sheet.Length+$sprite_atlas.hpbar_status.sprite_sheet.Length+$sprite_atlas.battle_hud.sprite_sheet.Length+$font_file.Length+$pokedex.Length))
}

for($battle_hud=0;$battle_hud -lt $sprite_atlas.battle_hud.sprite_sheet.Length;$battle_hud++) {
    $sprite_atlas.battle_hud.sprite_sheet[$battle_hud] = Convert-Sprite($sprite_atlas.battle_hud.sprite_sheet[$battle_hud])
    Write-Progress -Activity "Unpacking Battle hud Tiles..." -ID 1 -ParentId 0 -PercentComplete (100*$battle_hud/$sprite_atlas.battle_hud.sprite_sheet.Length)
    Write-Progress -Activity "Unpacking sprites..." -ID 0 -PercentComplete (100*($pokedex_tiles+$hpbar_status+$battle_hud+$letter+$mon)/($sprite_atlas.pokedex_tiles.sprite_sheet.Length+$sprite_atlas.hpbar_status.sprite_sheet.Length+$sprite_atlas.battle_hud.sprite_sheet.Length+$font_file.Length+$pokedex.Length))
}

for($letter=0;$letter -lt $font_file.Length;$letter++) {
    $alphabet.add(($font_file[$letter]).char, (Convert-Sprite($font_file[$letter].sprite)))
    Write-Progress -Activity "Unpacking Font..." -ID 1 -ParentId 0 -PercentComplete (100*$letter/$font_file.Length)
    Write-Progress -Activity "Unpacking sprites..." -ID 0 -PercentComplete (100*($pokedex_tiles+$hpbar_status+$battle_hud+$letter+$mon)/($sprite_atlas.pokedex_tiles.sprite_sheet.Length+$sprite_atlas.hpbar_status.sprite_sheet.Length+$sprite_atlas.battle_hud.sprite_sheet.Length+$font_file.Length+$pokedex.Length))
}

for($mon=0;$mon -lt $pokedex.Length;$mon++) {
    $pokedex[$mon].front_sprite = Convert-Sprite $pokedex[$mon].front_sprite
    $pokedex[$mon].back_sprite = Convert-Sprite $pokedex[$mon].back_sprite
    $pokedex[$mon].back_sprite = Resize-Sprite $pokedex[$mon].back_sprite -Scale 2
    $pokedex[$mon].back_sprite.height--
    Write-Progress -Activity "Unpacking Pokemon..." -ID 1 -ParentId 0 -PercentComplete (100*$mon/$pokedex.Length)
    Write-Progress -Activity "Unpacking sprites..." -ID 0 -PercentComplete (100*($pokedex_tiles+$hpbar_status+$battle_hud+$letter+$mon)/($sprite_atlas.pokedex_tiles.sprite_sheet.Length+$sprite_atlas.hpbar_status.sprite_sheet.Length+$sprite_atlas.battle_hud.sprite_sheet.Length+$font_file.Length+$pokedex.Length))
}

Export-ModuleMember -Variable sprite_atlas, alphabet, pokedex

Export-ModuleMember -Function Write-Screen
Export-ModuleMember -Function Convert-Sprite
Export-ModuleMember -Function Get-FileTime
Export-ModuleMember -Function Write-Text
Export-ModuleMember -Function Add-VBuff
Export-ModuleMember -Function Write-ScreenDebug
Export-ModuleMember -Function Resize-Sprite
Export-ModuleMember -Function Add-TextBox
Export-ModuleMember -Function Clear-TextBox
Export-ModuleMember -Function Clear-Screen