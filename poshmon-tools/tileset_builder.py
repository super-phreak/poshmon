from pokedata import Addr
from tileset import Tileset

import pokedata
import json

POKEDEX_TILESET = Addr(0x04,0x6488)
HPBAR_STATUS_GRAPHICS = Addr(0x04,0x5EA0)
BATTLE_HUDS = Addr(0x04,0x6080)

tilesets = dict()

tilesets["pokedex_tiles"] = Tileset.from_addr(POKEDEX_TILESET,"PokedexTileSet",18).to_json()
tilesets["hpbar_status"] = Tileset.from_addr(HPBAR_STATUS_GRAPHICS,"HPBarStatus",30).to_json()
tilesets["battle_hud"] = Tileset.from_addr(BATTLE_HUDS,"BATTLE_HUD",9,False).to_json()

with open('data/sprite_atlas.json', 'w') as sprite_atlas_file:
    json.dump(tilesets, sprite_atlas_file, indent=2)