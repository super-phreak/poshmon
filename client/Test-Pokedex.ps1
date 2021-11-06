param(
    [parameter(Mandatory=$false,ParameterSetName="PokedexIndex")]
    [Int]$PokedexIndex,

    [parameter(Mandatory=$false,ParameterSetName="InternalIndex")]
    [Int]$InternalIndex,

    [parameter(Mandatory=$false,ParameterSetName="Name")]
    [String]$Name,

    [parameter(Mandatory=$false)][Switch]
    $flip
)

function Decode-Sprite{
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
    return $sprite_raw
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
        $Flip
    )

    if ($flip) {
        $FLIP_SIGN = -1
        $FLIP_OFFSET = $sprite.width*$TILE_SIDE_RAW-1
    } else {
        $FLIP_SIGN = 1
        $FLIP_OFFSET = 0
    }
    $offset = $X + ($CANVAS_WIDTH*$Y)
    ##Temp function for now
    ##WHY ARRAY MATH SO HARD FOR ME SOMETIMES
    for ($index=0;$index -lt ($sprite.height*$sprite.width*$TILE_SIZE_RAW);$index++) {
            $v_buff[([MATH]::Floor($index/($sprite.width*$TILE_SIDE_RAW))*$CANVAS_WIDTH)+$offset+(($index%($sprite.width*$TILE_SIDE_RAW))*$FLIP_SIGN)+$FLIP_OFFSET] = $sprite.data[$index]
    }
}


#PoshMon Tests#
$Script:Logfile = "debug.log"
Set-Content -Path $Logfile -Value ""
$script:TILE_SIZE_RAW = 64
$script:TILE_SIDE_RAW = 8
$script:TILE_HIEGHT_POSH = 4

$script:colors = [enum]::GetValues([System.ConsoleColor])
$script:bufferCellType = [enum]::GetValues([System.Management.Automation.Host.BufferCellType])

$script:half_pixel = [char][int]"0x2584"
$script:poke_e = [char][int]"0x00e9"

$script:CANVAS_WIDTH = 160
$script:CANVAS_HEIGHT = 72

$script:v_buff = New-Object 'int[]' ($CANVAS_WIDTH * $CANVAS_HEIGHT * 2)

$script:gb_size = New-Object System.Management.Automation.Host.Size($CANVAS_WIDTH,$CANVAS_HEIGHT)
$debug_cursor_line = New-Object System.Management.Automation.Host.Coordinates 0, 71

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



Clear-Host
$Host.UI.RawUI.BackgroundColor = $($colors[[System.ConsoleColor]::Black])
$Host.UI.RawUI.BufferSize = $gb_size
$Host.UI.RawUI.WindowSize = $gb_size


$pokedex = Get-Content '../data/pokedex.json' | ConvertFrom-Json
$font_file = Get-Content '../data/font.json' | ConvertFrom-Json

$alphabet = New-Object -TypeName System.Collections.Hashtable


foreach($letter in $font_file) {
    $alphabet.add($letter.char, (Decode-Sprite($letter.sprite)))
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
}

$sprite_buffer = New-Object 'System.Management.Automation.Host.BufferCell[,]' ($CANVAS_HEIGHT, $CANVAS_WIDTH)

$sprite_raw = New-Object 'int[]'  ($target_mon.front_sprite.height*$target_mon.front_sprite.width*$TILE_SIZE_RAW)
$sprite_decoded = [System.Convert]::FromBase64String($target_mon.front_sprite.data)


for ($bytenum=0;$bytenum -lt $sprite_decoded.Length;$bytenum++) {
    for ($div=0;$div -lt 4;$div++) {
        $sprite_raw[($bytenum*4)+$div] = (($sprite_decoded[$bytenum] -shr (6 - ($div*2))) -band 3)
    }
}

$sprite_test_data = @{
    height = $target_mon.front_sprite.height
    width = $target_mon.front_sprite.width
    data = $sprite_raw
}

Add-VBuff -Sprite $sprite_test_data -X 0 -Y 0 -Flip

for ($row=0;$row -lt ($CANVAS_HEIGHT);$row++) {
    for ($col=0;$col -lt ($CANVAS_WIDTH);$col++) {
        $sprite_buffer[$row,$col] = $PIXELS[($v_buff[((($row * 2)    ) * $CANVAS_WIDTH) + $col] -shl 2) + 
                                             $v_buff[((($row * 2) + 1) * $CANVAS_WIDTH) + $col]]
    }
}


#Write-Host $v_buff[0..2000]
# Write-Host ($font | Where-Object {$_.char -ceq $target_mon.pokedex_entry.text[50]})
# Write-Host $alphabet['a']
# Write-Host $target_mon.name $target_mon.index
# Write-Host $target_mon.pokedex_entry.text
$coords = [System.Management.Automation.Host.Coordinates]::new(0,0)
$Host.UI.RawUI.SetBufferContents($coords,$sprite_buffer)
$Host.UI.RawUI.CursorPosition = $debug_cursor_line
