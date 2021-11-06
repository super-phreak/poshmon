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

font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+upper_punc+lower+1)),'é').to_json())
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+upper_punc+lower+2)),u"\u1E0B").to_json()) #ḋ to represent 'd as one letter
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+upper_punc+lower+3)),u"\u013A").to_json()) #ĺ to represent 'l as one letter
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+upper_punc+lower+4)),u"\u1E61").to_json()) #ṡ to represent 's as one letter
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+upper_punc+lower+5)),u"\u1E6B").to_json()) #ṫ to represent 't as one letter
font.append(Font(pokedata.FONT_START_POINTER+(pokedata.ONE_BPP_TILE_SIZE*(upper+upper_punc+lower+6)),u"\u1E7F").to_json()) #ṿ to represent 'v as one letter
        # 0xE0: "'",
        # 0xE1: u"\u1D18", #ᴘ to represent PK as one letter
        # 0xE2: u"\u1D0D", #ᴍ to represent MN as one letter
        # 0xE3: "-",
        # 0xE4: u"\u1E59", #ṙ to represent 'r as one letter
        # 0xE5: u"\u1E41", #ṁ to represent 'm as one letter


Sprite.decode_base64_sprite(font[-1]['sprite']['data'],1,1).print_pixels()

with open('data/font.json', 'w') as font_file:
    json.dump(font, font_file, indent=2)