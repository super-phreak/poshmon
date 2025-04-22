import pokedata

from tqdm import trange
from pokedata import Addr
from pokemon_entity import Pokemon


import base64
import json

pokedex = []
pbar = trange(len(pokedata.datamap['Index to Pokedex']))
for index in pbar:
    #Prevent MissingNo. and Mew from adding as they are not in the same address range
    if pokedata.datamap['Index to Pokedex'][index] > 0 and pokedata.datamap['Index to Pokedex'][index] != 151:
        pbar.set_description("Processing Pokemon: %s, %s" % pokedata.datamap['Index to Pokedex'][index], index)
        pokedex.append(Pokemon.from_addr(pokedata.POKEMON_DATA_POINTER+(28*(pokedata.datamap['Index to Pokedex'][index]-1)),index+1).to_json())

#Add Mew in After the fact
pokedex.append(Pokemon.from_addr(Addr(0x01,0x425B),21).to_json())

with open('../data/pokedex.json', 'w') as pokedex_file:
    json.dump(pokedex, pokedex_file, indent=2)