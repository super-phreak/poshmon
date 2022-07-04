param(
    [parameter(Mandatory=$true,ParameterSetName="PokedexIndex")]
    [Int]$PokedexIndex,

    [parameter(Mandatory=$true,ParameterSetName="InternalIndex")]
    [Int]$InternalIndex,

    [parameter(Mandatory=$true,ParameterSetName="Name")]
    [String]$Name,

    [parameter(Mandatory=$false)][Switch]
    $NoDisplay,

    [parameter(Mandatory=$false)][Switch]
    $NoClear,

    [parameter(Mandatory=$true,ParameterSetName="Random")][Switch]
    $Random,

    [parameter(Mandatory=$false)][int]
    $Scroll,

    [parameter(Mandatory=$false)][int]
    $ScrollTime = 10

)

function Exit-Poshmon {
    remove-module PoshmonGraphicsModule
    remove-module PoshdexModule
}

#PoshMon Tests#
Import-module .\PoshmonGraphicsModule.psm1
Import-module .\PoshdexModule

$poke_e = [char]0x00e9

if($PSBoundParameters.ContainsKey('PokedexIndex')) {
    $target_mon = $pokedex | Where-Object {$_.pokedex -eq $PokedexIndex}
    $pokedex = $pokedex | Sort-Object -Property {$_.pokedex}
    if ($null -eq $target_mon) {
        Write-Error "There is no Pok$($poke_e)mon with Pok$($poke_e)dex of $PokedexIndex"
        Exit-Poshmon
        exit 1
    }
} elseif($PSBoundParameters.ContainsKey('InternalIndex')) {
    $target_mon = $pokedex | Where-Object {$_.index -eq $InternalIndex}
    $pokedex = $pokedex | Sort-Object -Property {$_.index}
    if ($null -eq $target_mon) {
        Write-Error "There is no Pok$($poke_e)mon with index of $InternalIndex"
        Exit-Poshmon
        exit 1
    }
} elseif($PSBoundParameters.ContainsKey('Name')) {
    $target_mon = $pokedex | Where-Object {$_.name -eq $Name}
    if ($null -eq $target_mon) {
        Write-Error "There is no Pok$($poke_e)mon with name of $Name"
        Exit-Poshmon
        exit 1
    }
} elseif($PSBoundParameters.ContainsKey('Random')) {
    $ScrollRandom = $True
    $target_mon = $pokedex | Get-Random
} 

if($PSBoundParameters.ContainsKey('Scroll')) {
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
            $check = Show-DexEntry $target_mon -NoDisplay:$NoDisplay -Scroll -ScrollTime:$ScrollTime
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
            $check = Show-DexEntry $target_mon -NoDisplay:$NoDisplay -Scroll -ScrollTime:$ScrollTime
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
    Clear-Host
}

Exit-Poshmon