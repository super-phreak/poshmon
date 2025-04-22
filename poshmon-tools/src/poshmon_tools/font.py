#Needed because powershell ConvertFrom-Json is dumb and treats 'A' and 'a' as the same.

from pokedata import Sprite

class Font:
    def __init__(self,addr,char):
        self.addr = addr
        self.char = char
        self.sprite = Sprite.decode1BPP(addr,1,1)

    def to_json(self):
        return {
            'char' : self.char,
            'sprite' : self.sprite.to_json()
        }