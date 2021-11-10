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
            Add-VBuff -Sprite $internal_alphabet["$($print_text[$i])"] -x ($X+$i) -y $Y -TILE:$TILE
        }
    } else {
        Add-VBuff -Sprite $internal_alphabet["$($Text)"] -x $X -y $Y -TILE:$TILE
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

function Set-Alphabet {
    param(
        [parameter(Mandatory=$True)]
        $Alphabet
    )
    $script:internal_alphabet = $alphabet
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

Export-ModuleMember -Function Write-Screen
Export-ModuleMember -Function Convert-Sprite
Export-ModuleMember -Function Get-FileTime
Export-ModuleMember -Function Write-Text
Export-ModuleMember -Function Add-VBuff
Export-ModuleMember -Function Set-Alphabet
Export-ModuleMember -Function Write-ScreenDebug