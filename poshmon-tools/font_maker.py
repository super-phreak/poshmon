from pokedata import Addr, GBText as text, Sprite
from font import Font

import pokedata 
import json

font = []

list(map(lambda key: font.append(Font(pokedata.FONT_START_POINTER+((key-0x80)*pokedata.ONE_BPP_TILE_SIZE),text.ALPHABET[key]).to_json()), filter(lambda val: val >= 0x80, text.ALPHABET.keys())))

# #add blank space at the end
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*80))," ").to_json())

with open('data/font.json', 'w') as font_file:
    json.dump(font, font_file, indent=2)