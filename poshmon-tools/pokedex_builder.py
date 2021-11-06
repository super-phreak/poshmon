from pokedata import Addr
from pokemon_entity import Pokemon

import pokedata
import base64
import json

pokedex = []

for index in range(len(pokedata.datamap['Index to Pokedex'])):
    #Prevent MissingNo. and Mew from adding as they are not in the same address range
    if pokedata.datamap['Index to Pokedex'][index] > 0 and pokedata.datamap['Index to Pokedex'][index] != 151:
        print(pokedata.datamap['Index to Pokedex'][index], index)
        pokedex.append(Pokemon.from_addr(pokedata.POKEMON_DATA_POINTER+(28*(pokedata.datamap['Index to Pokedex'][index]-1)),index+1).to_json())

#Add Mew in After the fact
pokedex.append(Pokemon.from_addr(Addr(0x01,0x425B),21).to_json())

with open('data/pokedex.json', 'w') as pokedex_file:
    json.dump(pokedex, pokedex_file, indent=2)


#Debug Base 64 Sprites. To be implemented later
# base64_sprite = pokedex[0].front_sprite.to_base64()
# decoded_sprite_bytes = base64.b64decode(base64_sprite)

# print(base64_sprite)

# sprite_array = []

# for data in decoded_sprite_bytes:
#     for i in range(3,-1,-1):
#         sprite_array.append((data >> (i*2)) & 0b11)

# width = 5

# sprite = []
# for i in range(0,int(len(sprite_array)),width*8):
#     sprite.append(sprite_array[i:i+(width*8)])

# decoded_sprite = Sprite(Addr(0,0),width,width,sprite)

# decoded_sprite.print_pixels()