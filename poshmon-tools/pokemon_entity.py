from pokedata import Sprite, GBText, Addr, GBDataPacket as data
import pokedata
import move

class Pokemon:
    '''
    Pokemon data as found in the rom
        0x00	    Pokédex number	                byte
        0x01	    Base HP	                        byte
        0x02	    Base Attack	                    byte
        0x03	    Base Defense	                byte
        0x04	    Base Speed	                    byte
        0x05	    Base Special	                byte
        0x06	    Type 1	                        byte
        0x07	    Type 2	                        byte
        0x08	    Catch rate      	            byte
        0x09	    Base Exp. Yield	                byte
        0x0A	    Dimensions of frontsprite	    byte
        0x0B	    Pointer to frontsprite	        word
        0x0D	    Pointer to backsprite	        word
        0x0F-0x12	Attacks known at lv. 1	        4 bytes
        0x13	    Growth rate	                    byte
        0x14-0x1A	TM and HM flags	                7 bytes
        0x1B	    Padding	                        byte
    '''
    def __init__(self, addr, internal_index, pokedex_num, name,
                 base_hp, base_attack, base_defense, base_speed,
                 base_special, type1, type2, catch_rate, base_exp_yeild,
                 front_sprite, back_sprite, attacks_lvl_1, growth_rate, 
                 learnable_moves, evo_moves, evo_info, dex_entry) -> None:
        self.addr = addr
        self.internal_index = internal_index
        self.pokedex_num = pokedex_num
        self.name = name
        self.base_hp = base_hp
        self.base_attack = base_attack
        self.base_defense = base_defense
        self.base_speed = base_speed
        self.base_special = base_special
        self.type1 = type1
        self.type2 = type2
        self.catch_rate = catch_rate
        self.base_exp_yeild = base_exp_yeild
        self.front_sprite = front_sprite
        self.back_sprite = back_sprite
        self.attacks_lvl_1 = attacks_lvl_1
        self.growth_rate = growth_rate
        self.learnable_moves = learnable_moves
        self.evo_moves = evo_moves
        self.evo_info = evo_info
        self.dex_entry = dex_entry

    @classmethod
    def from_addr(cls, addr, internal_index) -> None:
        name = GBText(data.get_static_data(pokedata.POKEMON_NAME_POINTER+(pokedata.POKEMON_NAME_LENGTH*(internal_index-1)),pokedata.BYTE,pokedata.POKEMON_NAME_LENGTH))
        pokedex_num = data.get_static_data(addr,pokedata.BYTE,1).collapse()
        base_hp = data.get_static_data(addr+0x01,pokedata.BYTE,1).collapse()
        base_attack = data.get_static_data(addr+0x02,pokedata.BYTE,1).collapse()
        base_defense = data.get_static_data(addr+0x03,pokedata.BYTE,1).collapse()
        base_speed = data.get_static_data(addr+0x04,pokedata.BYTE,1).collapse()
        base_special = data.get_static_data(addr+0x05,pokedata.BYTE,1).collapse()
        type1 = data.get_static_data(addr+0x06,pokedata.BYTE,1).collapse()
        type2 = data.get_static_data(addr+0x07,pokedata.BYTE,1).collapse()
        catch_rate = data.get_static_data(addr+0x08,pokedata.BYTE,1).collapse()
        base_exp_yeild = data.get_static_data(addr+0x09,pokedata.BYTE,1).collapse()
        front_sprite = Sprite.parse_pkmn_sprite(Addr(bank=cls.__get_sprite_bank(internal_index),addr=data.get_static_data(addr+0x0B,pokedata.BYTE,2).collapse(rev=True)))
        back_sprite = Sprite.parse_pkmn_sprite(Addr(bank=cls.__get_sprite_bank(internal_index),addr=data.get_static_data(addr+0x0D,pokedata.BYTE,2).collapse(rev=True)))
        attacks_lvl_1 = data.get_static_data(addr+0x0F,pokedata.BYTE,4).data
        growth_rate = data.get_static_data(addr+0x13,pokedata.BYTE,1).collapse()
        learnable_moves = cls.__get_learnable_move_list(data.get_static_data(addr+0x14,pokedata.BYTE,7).collapse(rev=True))
        evo_info = cls.__get_evo_info(Addr(0x0E,pokedata.datamap['EVO Table'][internal_index-1]))
        dex_entry = PokedexEntry(Addr(0x10,pokedata.datamap['Pokedex Entry Loc'][internal_index-1]))
        return cls(addr, internal_index, pokedex_num, name, base_hp, base_attack, base_defense, base_speed, base_special, 
                   type1, type2, catch_rate, base_exp_yeild, front_sprite, back_sprite, attacks_lvl_1, 
                   growth_rate, learnable_moves, evo_info[0], evo_info[1], dex_entry)

    def __str__(self) -> str:
        return f"{self.pokedex_num:03},{self.internal_index:03},{self.base_hp:03},{self.dex_entry}"

    @classmethod
    def __get_sprite_bank(cls, index) -> int:
        '''
        ;       index < $1F:       bank $9 ("Pics 1")
        ; $1F ≤ index < $4A:       bank $A ("Pics 2")
        ; $4A ≤ index < $74:       bank $B ("Pics 3")
        ; $74 ≤ index < $99:       bank $C ("Pics 4")
        ; $99 ≤ index:             bank $D ("Pics 5")
        '''
        if   index == 0x15:
            return 0x01
        elif index < 0x1F:
            return 0x09
        elif index < 0x4A:
            return 0x0A
        elif index < 0x74:
            return 0x0B
        elif index < 0x99:
            return 0x0C
        else:
            return 0x0D

    @classmethod
    def __get_learnable_move_list(cls, moves_int) -> list:
        learnable_moves = []
        for i in range((7*pokedata.BYTE)-1):
            if ((moves_int >> i) & 1):
                learnable_moves.append(move.TM_HM_LIST[i])
        return learnable_moves
    
    @classmethod
    def __get_evo_info(cls, addr) -> tuple:
        evolutions = list()
        evo_moves = dict()
        evo_mon_bytes = data.get_var_data(addr,8,'0x00')
        evo_move_bytes = data.get_var_data(addr+len(evo_mon_bytes),8,'0x00')

        if len(evo_mon_bytes) > 1:
            i = 0
            while i < len(evo_mon_bytes)-1:
                evo_mon = dict()
                evo_mon['evo_method'] = evo_mon_bytes[i]

                if evo_mon['evo_method'] == 2:
                    evo_mon['evo_item_id'] = evo_mon_bytes[i+1]
                    i+=1

                evo_mon['evo_level'] = evo_mon_bytes[i+1]
                evo_mon['evo_mon_index'] = evo_mon_bytes[i+2]

                evolutions.append(evo_mon)
                i+=3
        

        for i in range(0,len(evo_move_bytes)-1,2):
            evo_moves[evo_move_bytes[i]] = evo_move_bytes[i+1]

        return (evo_moves,evolutions)


    

    def to_json(self) -> dict:
        return {
            'index': self.internal_index,
            'pokedex': self.pokedex_num,
            'name': str(self.name),
            'base_stats':{
                'hp': self.base_hp,
                'attack': self.base_attack,
                'defense': self.base_defense,
                'speed': self.base_speed,
                'special': self.base_special,
            },
            'types': [
                self.type1,
                self.type2
            ],
            'catch_rate': self.catch_rate,
            'base_exp_yeild': self.base_exp_yeild,
            'front_sprite': self.front_sprite.to_json(),
            'back_sprite': self.back_sprite.to_json(),
            'attacks_lvl_1': self.attacks_lvl_1,
            'growth_rate': self.growth_rate,
            'learnable_moves': self.learnable_moves,
            'pokedex_entry': self.dex_entry.to_json(),
            'evo_moves': self.evo_moves,
            'evo_info': self.evo_info
        }


class PokedexEntry:
    '''
        ; string: species name
        ; height in feet, inches
        ; weight in pounds
        ; text entry
    '''
    def __init__(self, addr) -> None:
        self.addr = addr
        self.species = GBText(data.get_var_data(addr,pokedata.BYTE,f"0x{GBText.STRING_END:#02x}"))
        h_feet_pkt = data.get_static_data(addr+len(self.species.packet.data),8,1)
        h_inches_pkt = data.get_static_data(h_feet_pkt.addr+len(h_feet_pkt.data),8,1)
        weight_pkt = data.get_static_data(h_inches_pkt.addr+len(h_inches_pkt.data),pokedata.BYTE,2)
        self.height = [h_feet_pkt.collapse(), h_inches_pkt.collapse()]
        self.weight = weight_pkt.collapse(rev=True)
        text_addr = data.get_static_data(weight_pkt.addr+len(weight_pkt.data)+1,pokedata.BYTE,2)
        text_bank = data.get_static_data(text_addr.addr+len(text_addr.data),pokedata.BYTE,1)
        if (Addr(bank=text_bank.collapse(),addr=text_addr.collapse(rev=True))<pokedata.END_FILE):
            self.text = GBText(data.get_var_data(Addr(bank=text_bank.collapse(),addr=text_addr.collapse(rev=True)),pokedata.BYTE,f"0x{GBText.STRING_END:#02x}"))
        else:
            self.text = "MISSING No. does<not exist.^Please look elsewere}"

    def __str__(self) -> str:
        height_str = f"{self.height[0]}' " + f'{self.height[1]}"'
        weight_str = f"{self.weight}lbs."
        return f"{{{self.addr} {str(self.species):<12} {height_str:<9} {weight_str:<10} {self.text}"

    def to_json(self) -> dict:
        return {'species': str(self.species), 'height': {'feet': self.height[0], 'inches': self.height[1]}, 'weight': self.weight, 'text': str(self.text)}