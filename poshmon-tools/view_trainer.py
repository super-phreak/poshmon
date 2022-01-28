from pokedata import Addr, GBDataPacket as data, Sprite
from pokemon_entity import Pokemon

import pokedata
import base64
import json

oak_addr = Addr(bank = 0x13, addr=0x615f)
bugcatcher_addr = Addr(bank = 0x13, addr=0x40c6)

oak_sprite = Sprite.parse_pkmn_sprite(bugcatcher_addr)

taggart_sprite_data = [
[2,2,2,2,2,2,2,0,0,2,0,0,0,0,0,0,2,0,2,2,2,2,2,2,2,0,0,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,0,3,3,3,0,0,0,0,0,0,0,0,0,0],
[2,0,0,0,0,0,2,0,2,2,0,2,2,2,0,2,2,0,2,0,0,0,0,0,2,0,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,3,3,2,3,3,0,0,0,0,0,0,0,0,0],
[2,0,2,2,2,0,2,0,0,2,2,2,0,2,0,0,2,0,2,0,2,2,2,0,2,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,3,3,3,3,2,3,3,0,0,0,0,0,0,0,0],
[2,0,2,2,2,0,2,0,2,2,0,2,2,2,2,2,0,0,2,0,2,2,2,0,2,0,0,0,0,0,0,3,3,3,0,1,1,1,1,1,1,1,1,0,0,3,2,3,0,0,0,0,0,0,0,0],
[2,0,2,2,2,0,2,0,0,0,2,0,2,0,2,2,2,0,2,0,2,2,2,0,2,0,0,0,0,0,3,3,3,1,2,2,2,2,2,2,2,2,2,2,0,0,3,3,3,0,0,0,0,0,0,0],
[2,0,0,0,0,0,2,0,2,0,2,0,2,2,2,2,2,0,2,0,0,0,0,0,2,0,0,0,0,0,3,3,3,2,3,3,3,3,3,3,3,3,3,3,2,1,0,3,3,0,0,0,0,0,0,0],
[2,2,2,2,2,2,2,0,2,0,2,0,2,0,2,0,2,0,2,2,2,2,2,2,2,0,0,0,0,3,0,3,1,2,3,1,1,3,3,3,3,1,1,2,3,2,1,3,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,2,2,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,2,2,1,1,1,1,3,3,1,1,1,1,1,2,0,3,0,0,0,0,0,0,0,0],
[2,2,2,2,2,0,2,2,2,2,2,0,2,0,2,2,0,2,0,2,0,2,0,2,0,0,0,0,0,0,0,2,2,1,2,1,1,1,1,1,1,1,3,3,0,2,2,1,0,0,0,0,0,0,0,0],
[2,0,2,2,2,0,0,0,2,2,0,0,0,0,2,0,2,0,2,2,0,0,0,0,2,0,0,0,0,0,1,2,2,1,3,3,3,1,0,1,1,3,3,0,0,0,2,2,1,0,0,0,0,0,0,0],
[0,0,2,0,2,2,2,0,0,2,0,0,0,2,2,0,0,2,2,2,2,0,2,0,0,0,0,0,0,0,2,2,2,1,0,0,3,3,0,1,3,3,0,0,0,0,2,2,2,0,0,0,0,0,0,0],
[2,0,2,0,2,2,0,0,2,2,2,2,0,0,2,0,0,2,0,2,0,0,0,0,0,0,0,0,0,0,2,2,2,1,0,0,3,1,0,1,2,3,0,0,0,0,2,2,2,0,0,0,0,0,0,0],
[0,0,2,2,2,2,2,0,2,2,0,0,2,0,2,2,0,2,2,2,2,2,0,2,2,0,0,0,0,0,1,2,2,1,1,0,0,1,0,0,0,0,0,0,0,0,2,2,1,0,0,0,0,0,0,0],
[2,2,2,2,0,0,0,0,0,0,2,2,2,0,0,2,0,2,0,2,2,0,0,0,0,0,0,0,0,0,0,1,2,2,1,1,1,1,1,0,0,0,0,0,0,2,2,1,0,0,0,0,0,0,0,0],
[2,0,0,2,0,0,2,2,0,2,2,0,0,2,2,0,0,2,2,0,2,0,2,0,0,0,0,0,0,0,0,0,0,3,3,1,1,1,1,0,0,2,0,0,0,3,0,0,0,0,0,0,0,0,0,0],
[2,0,0,0,0,0,0,0,0,0,2,2,0,0,2,0,2,0,0,0,0,0,0,2,2,0,0,0,0,0,0,0,0,0,3,1,1,3,3,3,3,1,0,0,2,0,0,0,0,0,0,0,0,0,0,0],
[2,0,2,0,0,0,2,2,2,2,0,0,0,0,2,2,2,2,2,2,2,2,0,0,2,0,0,0,0,0,0,0,0,0,0,3,1,1,1,1,1,1,0,3,0,0,0,0,0,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,2,0,2,0,2,0,0,0,2,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,3,3,3,1,1,1,1,1,3,3,3,0,0,0,0,0,0,0,0,0,0,0],
[2,2,2,2,2,2,2,0,2,0,0,0,2,2,2,0,2,0,2,0,2,0,2,2,2,0,0,0,0,0,0,0,3,3,1,0,0,3,3,3,3,3,0,3,0,3,0,0,0,0,0,0,0,0,0,0],
[2,0,0,0,0,0,2,0,0,0,0,2,0,0,0,2,2,0,0,0,2,2,0,0,0,0,0,0,0,0,3,3,1,1,0,0,3,3,3,1,0,0,0,3,0,0,3,3,0,0,0,0,0,0,0,0],
[2,0,2,2,2,0,2,0,2,2,2,0,0,0,0,2,2,2,2,2,2,2,0,0,0,0,0,0,0,3,3,3,3,1,0,0,3,3,3,3,3,3,3,0,0,0,2,0,3,2,0,0,0,0,0,0],
[2,0,2,2,2,0,2,0,2,0,2,2,0,0,2,2,2,0,0,2,2,2,0,2,2,0,0,0,0,3,0,0,0,3,3,0,3,3,0,0,0,0,0,3,0,0,2,0,0,0,2,0,0,0,0,0],
[2,0,2,2,2,0,2,0,2,2,2,0,2,2,2,0,0,2,2,2,2,0,2,0,2,0,0,0,0,3,3,3,0,0,0,3,0,0,0,3,3,3,3,3,2,2,2,0,0,0,3,0,0,0,0,0],
[2,0,0,0,0,0,2,0,2,0,0,2,0,0,0,2,0,2,2,0,2,2,2,2,0,0,0,0,3,3,2,2,3,3,3,3,3,3,3,2,2,2,2,3,2,2,0,0,0,0,0,2,0,0,0,0],
[2,2,2,2,2,2,2,0,2,2,2,0,0,2,2,0,0,2,0,0,0,2,2,2,2,0,0,0,3,0,3,2,3,2,3,0,0,3,2,2,2,2,2,3,0,0,2,0,0,0,2,3,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,3,3,0,3,0,0,3,2,2,2,2,2,2,3,0,0,0,2,0,2,0,3,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,3,3,0,3,0,3,3,3,3,2,2,2,2,3,0,0,2,0,0,3,0,3,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,3,3,3,3,3,3,0,0,0,0,0,1,3,2,2,2,3,0,0,2,0,0,3,0,0,2,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,3,0,0,0,0,3,3,3,2,2,2,2,3,0,2,0,0,0,3,0,0,3,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,3,1,0,0,0,0,3,2,2,3,3,3,3,0,2,0,0,0,3,0,0,3,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,3,3,3,0,0,0,3,3,3,0,0,3,0,2,0,0,0,0,3,0,0,3,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,0,3,3,3,1,3,0,0,0,0,0,3,1,2,0,0,0,0,3,0,0,0,3,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,3,3,3,3,3,0,0,0,0,0,3,1,0,0,0,0,0,3,0,0,0,3,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,3,3,3,3,1,1,3,3,0,0,0,0,0,0,3,1,0,0,0,0,0,3,1,0,0,3,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,3,3,0,3,3,3,0,0,3,3,3,0,0,0,0,0,0,3,1,1,3,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,3,2,3,3,3,3,2,3,0,0,0,0,0,0,3,1,1,3,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,3,3,3,0,0,3,3,3,0,0,0,0,2,2,3,1,3,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,0,2,2,0,0,0,3,2,2,3,3,2,2,3,3,0,0,2,0,0,0,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,2,0,0,3,2,2,3,2,2,2,2,3,0,0,2,0,0,0,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,2,0,0,3,2,2,3,2,2,2,2,3,0,0,2,0,0,0,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,2,0,0,3,2,3,3,2,2,2,2,3,0,0,2,0,0,0,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,2,0,0,3,2,2,2,3,2,2,2,3,0,0,2,0,0,0,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,2,0,0,3,2,2,3,0,3,2,2,3,0,0,2,0,0,0,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,2,0,0,2,0,0,3,2,2,3,0,3,2,2,3,0,0,2,0,0,2,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,2,2,2,0,0,3,2,2,3,0,3,2,2,3,0,0,2,2,2,0,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,0,0,0,0,3,2,2,1,3,0,3,2,2,3,0,0,0,0,0,0,3,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,3,0,0,0,3,2,1,1,3,0,3,2,2,3,0,0,0,0,3,3,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,3,3,3,3,2,1,3,0,0,0,3,2,3,3,3,3,3,3,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,2,2,2,2,1,1,3,0,0,0,3,2,2,2,2,2,2,3,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,2,2,2,1,1,1,3,0,0,0,3,2,2,2,1,1,1,3,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,2,2,2,2,1,1,3,0,0,0,3,2,1,1,3,3,2,3,0,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,2,3,3,3,2,2,2,2,3,0,0,3,2,2,2,2,2,2,2,3,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,2,3,0,3,1,1,3,3,3,3,3,3,2,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,1,0,0,0,3,0,0,3,0,0,0,3,3,3,1,0,0,0,3,3,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,1,1,1,1,1,3,3,0,0,0,0,0,3,1,1,1,1,0,0,3,0,0,0,0,0,0],
[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,3,3,3,3,0,0,0,0,0,0,0,0,3,3,3,3,3,3,0,0,0,0,0,0,0]
]
taggart_sprite = Sprite(None,7,7,taggart_sprite_data)

trainerdex = []

trainerdex.append(taggart_sprite.to_json())

with open('data/trainerdex.json', 'w') as trainerdex_file:
    json.dump(trainerdex, trainerdex_file, indent=2)