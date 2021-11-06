from pokedata import GBDataPacket as data, Sprite, Addr
from font import Font

import pokedata 
import json

A = ord('A')
Z = ord('Z')

a = ord('a')
z = ord('z')

upper_punc = 7

font = []

for upper in range(Z-A+1):
    font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*upper),chr(A+upper)).to_json())

font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+1)),'(').to_json())
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+2)),')').to_json())
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+3)),':').to_json())
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+4)),';').to_json())
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+5)),'[').to_json())
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+6)),']').to_json())

for lower in range(z-a+1):
    font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+upper_punc+lower)),chr(a+lower)).to_json())

font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+upper_punc+lower)),'é').to_json())


with open('data/font.json', 'w') as font_file:
    json.dump(font, font_file, indent=2)