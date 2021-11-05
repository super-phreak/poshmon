from pokedata import GBDataPacket as data, GBText as text, Addr
from move import Move

import move
import pokedata 
import json

moves = []
start_addr = Addr(0xE,0x4000)

for i in range(165):
    moves.append(Move.from_addr(addr=start_addr+(i*move.MOVE_DATA_LENGTH)).to_json())

with open('data/moves.json', 'w') as moves_file:
    json.dump(moves, moves_file, indent=2)