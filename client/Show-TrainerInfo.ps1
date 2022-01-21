param(
    [parameter(Mandatory=$false)][Switch]
    $NoDisplay,

    [parameter(Mandatory=$false)][Switch]
    $NoClear
)

function Exit-Poshmon {
    remove-module PoshmonGraphicsModule
}

Import-module .\PoshmonGraphicsModule.psm1

$trainerdex = Get-Content '../data/trainerdex.json' | ConvertFrom-Json

$trainerdex[0] = Convert-Sprite $trainerdex[0]

Add-VBuff -Sprite $trainerdex[0], -x 12 -y 0 -Tile
Write-Screen

Exit-Poshmon