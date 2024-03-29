from pokedata import GBText as text, GBDataPacket as data

import pokedata

class Move:
    def __init__(self, addr, id, name, effect, power, move_type, accuracy, pp, priority) -> None:
        self.addr = addr
        self.id = id
        self.name = name
        self.effect = effect
        self.power = power
        self.move_type = move_type
        self.accuracy = accuracy
        self.pp = pp
        self.priority = priority

    @classmethod
    def from_addr(cls, addr):
        id = data.get_static_data(addr, pokedata.BYTE, 1).collapse()
        effect = data.get_static_data(addr+1, pokedata.BYTE, 1).collapse()
        power = data.get_static_data(addr+2, pokedata.BYTE, 1).collapse()
        move_type = data.get_static_data(addr+3, pokedata.BYTE, 1).collapse()
        accuracy = data.get_static_data(addr+4, pokedata.BYTE, 1).collapse()
        pp = data.get_static_data(addr+5, pokedata.BYTE, 1).collapse()
        if id == 98:
            priority = 1
        elif id == 68:
            priority = -1
        else:
            priority = 0
        return cls(addr,id,MOVE_NAMES[id],effect,power,move_type,accuracy,pp, priority)

    def to_json(self) -> dict:
        return {
            "id": self.id,
            "name": self.name,
            "effect": self.effect,
            "power": self.power,
            "type_id": self.move_type,
            "accuracy": self.accuracy,
            "pp": self.pp,
            "priority": self.priority
        }

        
MOVE_DATA_LENGTH = 6
MOVE_NAMES = []

MOVE_NAMES.append("NO MOVE")

offset = 0

for i in range(165):
    move = text(data.get_var_data(pokedata.MOVE_NAME_POINTER + offset,8,f"0x{text.STRING_END:#02x}"))
    offset += len(move)
    MOVE_NAMES.append(str(move))

TM_HM_LIST = []

# TM_HM_LIST.append(0)

for i in range(55):
    TM_HM_LIST.append(data.get_static_data(pokedata.TM_HM_LIST_POINTER+i,pokedata.BYTE,1).collapse())