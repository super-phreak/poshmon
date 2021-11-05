param(
    [parameter(Mandatory=$false)][ValidateRange(1,151)]
    [Int]$pokedexnum,

    [parameter(Mandatory=$false)][Switch]
    $flip
)

if(!$PSBoundParameters.ContainsKey('pokedexnum')) {
    $pokedexnum = Get-Random -Minimum 1 -Maximum 151
}

#PoshMon Tests#
$TILE_SIZE_RAW = 64
$TILE_SIDE_RAW = 8
$TILE_HIEGHT_POSH = 4

Clear-Host

$half_pixel = [char][int]"0x2584"

$gb_size = New-Object System.Management.Automation.Host.Size(160,72)
$debug_cursor_line = New-Object System.Management.Automation.Host.Coordinates 0, 71
$Host.UI.RawUI.BufferSize = $gb_size
$Host.UI.RawUI.WindowSize = $gb_size


$colors = [enum]::GetValues([System.ConsoleColor])
$bufferCellType = [enum]::GetValues([System.Management.Automation.Host.BufferCellType])

$Host.UI.RawUI.BackgroundColor = $($colors[[System.ConsoleColor]::Black])

$pokedex = Get-Content '../data/pokedex.json' | ConvertFrom-Json
$target_mon = $pokedex | Where-Object {$_.pokedex -eq $pokedexnum}

$logo = [char[]] '000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002fb1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001bfdef20000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000017fd555df71000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011101200000000003fe55567bed400000000000000000000000000000000000000000000000000000000000000000112223332221000000000000000001237bfff51bfb20000001bfa67bfeea10000122222227fffffff600000000000000000000000000000000000000000123bbffeedddddeeffb2100000000007bffedd99ef6bf9dfb300006afffffbbbb50000bfeddddefff5555fb000000000abb7321100000000000000000000127bfedd95555555555559def7100000000ff5555555affe5555def727ffed99999ddff711ff555555efd5555ff000000000afddddeefbb7012110000000001bfed9555555555555555555559ff2000006aefff55555af95555555affe9555666655559fbbfa5555559f55555ef6000000aaaf76555555ef5afeffb77321001afb655555555555566666555559fb10000aaaeef655559955555557ff95556bfffd5556bfeff955555559555555fb001122aaaefa5555559fffa555599fe41aaaaff6555555555559fffff75555ff50112baaaafa555555555557fffa5555fedd9556bffa9ff555555555555555ff7bfeddddeffa5555555fff555555bf9009aaaaef6676555555559ddeffa555ff7bedddddeffb5555555557beaff55555e55556bfedefbfe555555555555555bfd9566555559fb555555afe555556fe00009aaaaeffff655555555555af955bffd5566555559fb55555559efb7ff6555555559dd55559efb75555555555555bfd55bf55556559fa555555f555555ff4000004aaaaaaaff65555555556d556bfd555bf55555559fb55555555559def75555555555555567fe9555665555b55bf9555efb777f555ef555555955555bf900000004aaaaaaaff555555555556bff95555eff777f555af5556655555555dfb776555555677bffe55555af6556f65ff55555deeed5555fe555555555556fe0000000004804aaaaff555555556bfeef555555deeed5555bf555fffb765555559deffffffffeeaff955555bff65ffb5ff6555555555555bf555a55555555ff400000000000008aaaafb5555555efaaaf65555555555555bf9555ffaeeffb755555555efaaaaaaaff766656faefffff59fb65555555556fe5556f5555555bf90000000000000009aaaefb5555555ffaaef655555555556bff5555ffaaaaaaeefb76555afaaa98aaaeeeeeeefaaafeff559efbb77777bff95555bf5555556fe000000000000000009aaaefb555555efaaaefb76555567bfeff5555ff9aaaaaaaaaeffb7bf50001aaaaaaaaa8aaaaaaff55555dffeeeeaff55555ff555555bf40000000000000000009aaaef7555555ffaaaaeefffffeeaaaafbbffff50489aaaaaaaaaeef6000088999aaaa008a8aaefffffffffaaaaaffb777bff55555af900000000000000000000aaaaef655555afaaaaaaaaaaaaaaaaaaaaaa50000000489aaaaaa544000000000000000000aaaaaaaaaa99988aaaaaaaeeff55556fe0000000000000000000000aaaaef655677ff58aaaaaaa9840aaaaaaaa500000000000489aa60000000000000000000099aaaaaaaa50000aaaaaaaaaff7766bf4cfcf303f000000000000004aaaaffffeed88400044440000088844000000000000000000044000000000000000000000000004488400004488889aaaaaeeef900f0f0c0f0000000000000004aaaaeeaaaa10000000000000000000000000000000000000000000000000000000000000000000000000000000005aaaaaaaa4000000000000000000000000004aaaaaa9884000000000000000000000000000000000000000000000000000000000000000000000000000000000048899aa900000000000000000000000000004884000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000'

$PIXELS = @(
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

$sprite_buffer = New-Object 'System.Management.Automation.Host.BufferCell[,]' ($target_mon.front_sprite.height*4), ($target_mon.front_sprite.width*8)

$sprite_raw = New-Object 'int[]'  ($target_mon.front_sprite.height*$target_mon.front_sprite.width*$TILE_SIZE_RAW)
$sprite_decoded = [System.Convert]::FromBase64String($target_mon.front_sprite.data)

for ($bytenum=0;$bytenum -lt $sprite_decoded.Length;$bytenum++) {
    for ($div=0;$div -lt 4;$div++) {
        $sprite_raw[($bytenum*4)+$div] = (($sprite_decoded[$bytenum] -shr (6 - ($div*2))) -band 3)
    }
}

if ($flip) {
    $FLIP_SIGN = -1
    $FLIP_OFFSET = 1-$target_mon.front_sprite.width*$TILE_SIDE_RAW
} else {
    $FLIP_SIGN = 1
    $FLIP_OFFSET = 0
}

##Temp function for now
##WHY ARRAY MATH SO HARD FOR ME SOMETIMES
for ($row=0;$row -lt ($target_mon.front_sprite.height*$TILE_HIEGHT_POSH);$row++) {
    for ($col=0;$col -lt ($target_mon.front_sprite.width*$TILE_SIDE_RAW);$col++) {
        $sprite_buffer[$row,$col] = $PIXELS[($sprite_raw[((($row * 2)    ) * $target_mon.front_sprite.width*$TILE_SIDE_RAW) + (($FLIP_OFFSET+$col)*$FLIP_SIGN)] -shl 2) + 
                                             $sprite_raw[((($row * 2) + 1) * $target_mon.front_sprite.width*$TILE_SIDE_RAW) + (($FLIP_OFFSET+$col)*$FLIP_SIGN)]]
    }
}

$coords = [System.Management.Automation.Host.Coordinates]::new(16,4)
$Host.UI.RawUI.SetBufferContents($coords,$sprite_buffer)
$Host.UI.RawUI.CursorPosition = $debug_cursor_line