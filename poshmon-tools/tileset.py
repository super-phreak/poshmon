from pokedata import Sprite

import pokedata

class Tileset:
    def __init__(self,addr,name,sprite_sheet):
        self.addr = addr
        #Not sure if I want the name apart of the class or on the external dict yet.
        self.name = name
        self.sprite_sheet = sprite_sheet

    def to_json(self):
        sprite_sheet_json = list(map(lambda sprite: sprite.to_json(),self.sprite_sheet))
        return {
            'name' : self.name,
            'sprite_sheet' : sprite_sheet_json
        }

    @classmethod
    def from_addr(cls,addr,name,tiles):
        sprite_sheet = []
        for i in range(tiles):
            sprite_sheet.append(Sprite.decode2BPP(addr+(i*pokedata.TWO_BPP_TILE_SIZE),1,1))
        return cls(addr,name,sprite_sheet)