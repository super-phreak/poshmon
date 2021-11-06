from bitstring import Bits, BitString, BitArray, ConstBitStream

import base64

class Addr:
    def __init__(self,bank,addr) -> None:
        self.bank = bank
        self.addr = addr

    def absolute_pos(self) -> int:
        return (((self.bank-1)*BANK_SIZE)+self.addr)

    @classmethod
    def convert_to_addr(cls, long_addr) -> None:
        bank = int(long_addr/BANK_SIZE)
        addr = (long_addr%BANK_SIZE)+(BANK_SIZE if bank > 0 else 0)
        return cls(bank,addr)
    
    def __str__(self) -> str:
        return f"{self.bank:#04X}:{self.addr:04X}"

    def __add__(self, other):
        if isinstance(other, int):
            diff = other
        elif isinstance(other, Addr):
            diff = abs(self.absolute_pos() - other.absolute_pos())
        return self.convert_to_addr(self.absolute_pos() + diff)

    def __sub__(self, other):
        if isinstance(other, int):
            diff = other
        elif isinstance(other, Addr):
            diff = abs(self.absolute_pos() - other.absolute_pos())
        return self.convert_to_addr(self.absolute_pos() - diff)

    def __eq__(self, other) -> bool:
        return self.absolute_pos() == other.absolute_pos()

    def __gt__(self, other) -> bool:
        return self.absolute_pos() > other.absolute_pos()
    
    def __lt__(self, other) -> bool:
        return self.absolute_pos() < other.absolute_pos()
    
    def __ge__(self, other) -> bool:
        return self.absolute_pos() >= other.absolute_pos()
    
    def __le__(self, other) -> bool:
        return self.absolute_pos() <= other.absolute_pos()
    
    def __ne__(self, other) -> bool:
        return self.absolute_pos() != other.absolute_pos()

class GBDataPacket:
    def __init__(self, addr, packet_size, data) -> None:
        self.addr = addr
        self.packet_size = packet_size
        self.data = data
    
    @classmethod
    def get_static_data(cls, addr, packet_size, length):
        ROM.bytepos = addr.absolute_pos()
        data = ROM.readlist([f'uint:{packet_size}']*length)
        return cls(addr,packet_size,data)

    @classmethod
    def get_var_data(cls, addr, packet_size, target, bytealigned=True):
        ROM.bytepos = addr.absolute_pos()
        data = ROM.readto(target,bytealigned)
        data_list = data.readlist([f'uint:{packet_size}']*int(data.len/packet_size))
        return cls(addr,packet_size,data_list)

    def collapse(self, rev=False) -> int:
        out = 0
        if rev:
            self.data.reverse()
        for val in self.data:
            out = out << self.packet_size
            out+=val
        if rev:
            self.data.reverse()
        return out

    def __str__(self) -> str:
        return f"{self.addr}  " + " ".join(map((lambda n: f"{n:02x}"), self.data))

    def raw_dump(self) -> str:
        out = ""
        out+=(f"Start:{self.addr} Finish:{self.addr+len(self.data)} Length:{(len(self.data))} 2BPP:{len(self.data)/16:0.0f} 1BPP:{len(self.data)/8:0.0f}\n")
        

        data_fmt = []
        for i in range(int(len(self.data)/16)):
            data_fmt.append(f"{(i*16):#07X} " + ' '.join(map(lambda n: f"{n:02X}", self.data[16*i:(16*i)+16])))

        out+=('\n'.join(data_fmt))
        if (len(self.data) % 16 != 0):
            out+=(f"\n{len(data_fmt)*16:#07X} " + ' '.join(map(lambda n: f"{n:02X}", self.data[len(data_fmt)*16:])))
        return out
    
    def __len__(self):
        return len(self.data)

class Sprite:

    def __init__(self,addr,width,height,data) -> None:
        self.addr = addr
        self.width = width
        self.height = height
        self.data = data

    def __str__(self):
        return f"[Loc: {self.addr} => Width: {self.width}, Height: {self.height}]"

    def to_json(self) -> dict:
        return {'width': self.width, 'height': self.height, 'data': self.to_base64()}

    @classmethod
    def __expandRLEPacket(cls, bit_length, value) -> BitString:
        return BitString((bit_length+value+1)*2)

    @classmethod
    def __findRLEBoundry(cls, sprite_data) -> Bits:
        length_found = sprite_data.readto('0b0')
        return length_found

    @classmethod
    def __mode1(cls,bit_planes,width) -> list:
        bit_planes[1] = cls.__deltaDecode(bit_planes[1],width)
        bit_planes[0] = cls.__deltaDecode(bit_planes[0],width)
        return bit_planes

    @classmethod
    def __mode2(cls,bit_planes,width) -> list:
        bit_planes[1] = cls.__deltaDecode(bit_planes[1],width)
        bit_planes[0] = bit_planes[0] ^ bit_planes[1] 
        return bit_planes

    @classmethod
    def __mode3(cls,bit_planes,width) -> list:
        bit_planes[1] = cls.__deltaDecode(bit_planes[1],width)
        bit_planes[0] = cls.__deltaDecode(bit_planes[0],width)
        bit_planes[0] = bit_planes[0] ^ bit_planes[1]
        return bit_planes

    @classmethod
    def ___translate(cls, arr,row_num,coloumn_num):
        matrix = [[0 for x in range(coloumn_num)] for y in range(row_num)]
        for row in range(row_num):
            for col in range(int(coloumn_num/8)):
                for i in range(8):
                    matrix[row][col+i]=arr[(row*col)+row+i]
        return matrix


    @classmethod
    def __fillMatrix(cls, arr,row_num, coloumn_num) -> BitArray:
        #Array math is hard touch numbers at own risk
        matrix = [[0 for x in range(coloumn_num*4)] for y in range(row_num*8)]
        for row in range(row_num*8):
            for col in range(coloumn_num*4):
                matrix[row][col]=(''.join(arr[((col*row_num*16)+(row*2)):((col*row_num*16)+(row*2))+2].bin))
            matrix[row] = ''.join(matrix[row])
        
        output = BitArray()
        for out_row in matrix:    
            output.append('0b'+out_row)

        return output

    @classmethod
    def __bufferToList(cls, arr, row_num, coloumn_num) -> list:
        #1 byte per row per tile
        #1 byte per coloumn per tile
        bufList = [0] * row_num*BYTE
        column_bits = coloumn_num*BYTE
        for row in range(row_num*BYTE):
            bufList[row]=list(map(int,(','.join(arr[(row*column_bits):((row*column_bits)+column_bits)].bin).split(','))))
        return bufList

    @classmethod
    def __combineBuffers(cls,bit_planes,high_bit_plane) -> list:
        result = [[(bit_planes[high_bit_plane][i][j]<<1) + bit_planes[high_bit_plane^1][i][j]  for j in range(len(bit_planes[high_bit_plane][0]))] for i in range(len(bit_planes[1]))]
        return result

    @classmethod
    def __fillTileMatrix(cls, arr, sprite_height_tiles, sprite_width_tiles) -> list:
        tile_side_px = 8
        tile_size = tile_side_px*tile_side_px
        out = []
        for tile_row in range (sprite_height_tiles):
            for row in range(tile_side_px):
                temp = []
                for col in range (sprite_width_tiles):
                    temp.extend(arr[((tile_row*tile_size*sprite_width_tiles)+(col*tile_size)+(row*tile_side_px)):((tile_row*tile_size*sprite_width_tiles)+(col*tile_size)+(row*tile_side_px))+tile_side_px])
                out.append(temp)
        return out

    def print_pixels(self):
        for row in self.data:
            print(','.join(map(str,row)))

    def __to_bignum(self) -> int:
        output = 0
        for row in self.data:
            for col in row:
                output = output << 2
                output += col
        return output

    def to_base64(self) -> str:
        num = self.__to_bignum()
        num_bytes = num.to_bytes((int(self.height*self.width*TWO_BPP_TILE_SIZE)),'big')
        return base64.b64encode(num_bytes).decode()
    
    @classmethod
    def __deltaDecode(cls, arr, width) -> BitArray:
        output = BitArray()
        currentBit = 0
        for index, bit in enumerate(arr):
            if index % (width*8) == 0:
                currentBit = 0
            if bit:
                currentBit = (currentBit ^ 1)
            
            output.append('0b%s' % currentBit)
        return output

    @classmethod
    def __parseData(cls, packet_type, width, height, bit_plane):
        while bit_plane.len < (width*height*ONE_BPP_TILE_SIZE*BYTE):
            if packet_type == 0:
                length = cls.__findRLEBoundry(ROM)
                value = ROM.read((f"uint:{length.len}"))
                zero_bits = cls.__expandRLEPacket(length.uint,value)
                bit_plane.append(zero_bits)
                packet_type = 1
            else:
                data_packet = ROM.read('bin:2')
                if data_packet != '00':
                    bit_plane.append('0b'+data_packet)
                else:
                    packet_type = 0

    @classmethod
    def parse_pkmn_sprite(cls, addr) -> None:
        ROM.bytepos = addr.absolute_pos()
        width = ROM.read('uint:4')
        height = ROM.read('uint:4')
        high_bit_plane = ROM.read('uint:1')
        packet_type = ROM.read('uint:1')
        bit_planes = [BitArray(), BitArray()]
        cls.__parseData(packet_type,width,height,bit_planes[1])
        zip_mode = -1
        if ROM.peek('uint:1') == 0:
            zip_mode = ROM.read('uint:1')
        else:
            zip_mode = ROM.read('uint:2')
        packet_type = ROM.read('uint:1')

        cls.__parseData(packet_type,width,height,bit_planes[0])

        bit_planes[0] = cls.__fillMatrix(bit_planes[0],width,height)
        bit_planes[1] = cls.__fillMatrix(bit_planes[1],width,height)
        if zip_mode == 0:
            bit_planes = cls.__mode1(bit_planes,width)
        elif zip_mode == 2:
            bit_planes = cls.__mode2(bit_planes,width)
        else:
            bit_planes = cls.__mode3(bit_planes,width)

        bit_planes[0] = cls.__bufferToList(bit_planes[0],width,height)
        bit_planes[1] = cls.__bufferToList(bit_planes[1],width,height)

        sprite_data = cls.__combineBuffers(bit_planes,high_bit_plane)

        return cls(addr,width,height,sprite_data)

    @classmethod
    def decode1BPP(cls,addr,width,height):
        ROM.bytepos = addr.absolute_pos()
        bit_planes = [BitArray(), BitArray()]
        for i in range(width*height*BYTE):
            bit_planes[0].append(ROM.peek('bits:8'))
            bit_planes[1].append(ROM.read('bits:8'))
        
        for i in range(2):
            bit_planes[i] = cls.__fillTileMatrix(bit_planes[i],height,width)

        sprite_data = cls.__combineBuffers(bit_planes,1)
        
        return cls(addr,width,height,sprite_data)


    @classmethod
    def decode2BPP(cls,addr,width,height):
        ROM.bytepos = addr.absolute_pos()
        bit_planes = [BitArray(), BitArray()]
        for i in range(width*height*BYTE*2):
            bit_planes[0].append(ROM.read('bits:8'))
            bit_planes[1].append(ROM.read('bits:8'))
        
        for i in range(2):
            bit_planes[i] = cls.__fillTileMatrix(bit_planes[i],height,width)

        sprite_data = cls.__combineBuffers(bit_planes,0)
        
        return cls(addr,width,height,sprite_data)


class GBText:
    STRING_END = 0x50
    ALPHABET = {
        0x00: "",           #charmap "<NULL>"
        0x49: "^",       #charmap "<PAGE>"
        #charmap "<PKMN>",    #  "<PK><MN>"
        #charmap "<_CONT>",   #  implements "<CONT>"
        #charmap "<SCROLL>",  $4c
        0x4E: ">",     #Next
        0x4F: " ",   
        0x57: "#",
        0x50: "@",   #charmap "@" string terminator
        0x51: "*",
        0x52: "A1",
        0x53: "A2",
        0x54: "POKé", #This is fine to leave multichar as it was only short hand for all four characters anyway
        0x55: "+",
        0x58: "$",
        0x5F: "}",   #charmap "<DEXEND>"
        0x75: "…",
        0x7F: " ",
        0x80: "A",
        0x81: "B",
        0x82: "C",
        0x83: "D",
        0x84: "E",
        0x85: "F",
        0x86: "G",
        0x87: "H",
        0x88: "I",
        0x89: "J",
        0x8A: "K",
        0x8B: "L",
        0x8C: "M",
        0x8D: "N",
        0x8E: "O",
        0x8F: "P",
        0x90: "Q",
        0x91: "R",
        0x92: "S",
        0x93: "T",
        0x94: "U",
        0x95: "V",
        0x96: "W",
        0x97: "X",
        0x98: "Y",
        0x99: "Z",
        0x9A: "(",
        0x9B: ")",
        0x9C: ":",
        0x9D: ";",
        0x9E: "[",
        0x9F: "]",
        0xA0: "a",
        0xA1: "b",
        0xA2: "c",
        0xA3: "d",
        0xA4: "e",
        0xA5: "f",
        0xA6: "g",
        0xA7: "h",
        0xA8: "i",
        0xA9: "j",
        0xAA: "k",
        0xAB: "l",
        0xAC: "m",
        0xAD: "n",
        0xAE: "o",
        0xAF: "p",
        0xB0: "q",
        0xB1: "r",
        0xB2: "s",
        0xB3: "t",
        0xB4: "u",
        0xB5: "v",
        0xB6: "w",
        0xB7: "x",
        0xB8: "y",
        0xB9: "z",
        0xBA: "é",
        0xBB: u"\u1E0B", #ḋ to represent 'd as one letter
        0xBC: u"\u013A", #ĺ to represent 'l as one letter
        0xBD: u"\u1E61", #ṡ to represent 's as one letter
        0xBE: u"\u1E6B", #ṫ to represent 't as one letter
        0xBF: u"\u1E7F", #ṿ to represent 'v as one letter
        0xE0: "'",
        0xE1: u"\u1D18", #ᴘ to represent PK as one letter
        0xE2: u"\u1D0D", #ᴍ to represent MN as one letter
        0xE3: "-",
        0xE4: u"\u1E59", #ṙ to represent 'r as one letter
        0xE5: u"\u1E41", #ṁ to represent 'm as one letter
        0xE6: "?",
        0xE7: "!",
        0xE8: ".",
        0xED: "→",
        0xEE: "↓",
        0xEF: "♂",

        0x60: "<BOLD_A>",  #  unused
        0x61: "<BOLD_B>",  #  unused
        0x62: "<BOLD_C>",  #  unused
        0x63: "<BOLD_D>",  #  unused
        0x64: "<BOLD_E>",  #  unused
        0x65: "<BOLD_F>",  #  unused
        0x66: "<BOLD_G>",  #  unused
        0x67: "<BOLD_H>",  #  unused
        0x68: "<BOLD_I>",  #  unused
        0x69: "<BOLD_V>",  
        0x6A: "<BOLD_S>",  
        0x6B: "<BOLD_L>",  #  unused
        0x6C: "<BOLD_M>",  #  unused
        0x6D: "<COLON>",   #  colon with tinier dots than ":"
        0x6E: "ぃ",         #  hiragana small i, unused
        0x6F: "ぅ",         #  hiragana small u, unused
        0x70: "‘",         #  opening single quote
        0x71: "’",         #  closing single quote
        0x72: "“",         #  opening quote
        0x73: "”",         #  closing quote
        0x74: "·",         #  middle dot, unused
        0x75: "…",         #  ellipsis
        0x76: "ぁ",         #  hiragana small a, unused
        0x77: "ぇ",         #  hiragana small e, unused
        0x78: "ぉ",         #  hiragana small o, unused


        0x79: "┌",         
        0x7A: "─",         
        0x7B: "┐",         
        0x7C: "│",         
        0x7D: "└",         
        0x7E: "┘",         
        0x7F: " ",         

        0x05: "ガ", 
        0x06: "ギ",
        0x07: "グ", 
        0x08: "ゲ", 
        0x09: "ゴ", 
        0x0A: "ザ", 
        0x0B: "ジ", 
        0x0C: "ズ", 
        0x0D: "ゼ", 
        0x0E: "ゾ", 
        0x0F: "ダ", 
        0x10: "ヂ", 
        0x11: "ヅ", 
        0x12: "デ", 
        0x13: "ド", 

        0x19: "バ", 
        0x1A: "ビ", 
        0x1B: "ブ", 
        0x1C: "ボ", 

        0x26: "が", 
        0x27: "ぎ", 
        0x28: "ぐ", 
        0x29: "げ", 
        0x2A: "ご", 
        0x2B: "ざ", 
        0x2C: "じ", 
        0x2D: "ず", 
        0x2E: "ぜ", 
        0x2F: "ぞ", 
        0x30: "だ", 
        0x31: "ぢ", 
        0x32: "づ", 
        0x33: "で", 
        0x34: "ど", 

        0x3A: "ば", 
        0x3B: "び", 
        0x3C: "ぶ", 
        0x3D: "べ", 
        0x3E: "ぼ", 

        0x40: "パ", 
        0x41: "ピ", 
        0x42: "プ", 
        0x43: "ポ", 
        0x44: "ぱ", 
        0x45: "ぴ", 
        0x46: "ぷ", 
        0x47: "ぺ", 
        0x48: "ぽ", 

        0x70: "「", 
        0x71: "」", 
        0x73: "』", 
        0x75: "⋯", 

        0x7F: " ", 

        # 0x80: "ア", 
        # 0x81: "イ", 
        # 0x82: "ウ", 
        # 0x83: "エ", 
        # 0x84: "オ", 
        # 0x85: "カ", 
        # 0x86: "キ", 
        # 0x87: "ク", 
        # 0x88: "ケ", 
        # 0x89: "コ", 
        # 0x8A: "サ", 
        # 0x8B: "シ", 
        # 0x8C: "ス", 
        # 0x8D: "セ", 
        # 0x8E: "ソ", 
        # 0x8F: "タ", 
        # 0x90: "チ", 
        # 0x91: "ツ", 
        # 0x92: "テ", 
        # 0x93: "ト", 
        # 0x94: "ナ", 
        # 0x95: "ニ", 
        # 0x96: "ヌ", 
        # 0x97: "ネ", 
        # 0x98: "ノ", 
        # 0x99: "ハ", 
        # 0x9A: "ヒ", 
        # 0x9B: "フ", 
        # 0x9C: "ホ", 
        # 0x9D: "マ", 
        # 0x9E: "ミ", 
        # 0x9F: "ム", 
        # 0xA0: "メ", 
        # 0xA1: "モ", 
        # 0xA2: "ヤ", 
        # 0xA3: "ユ", 
        # 0xA4: "ヨ", 
        # 0xA5: "ラ", 
        # 0xA6: "ル", 
        # 0xA7: "レ", 
        # 0xA8: "ロ", 
        # 0xA9: "ワ", 
        # 0xAA: "ヲ", 
        # 0xAB: "ン", 
        # 0xAC: "ッ", 
        # 0xAD: "ャ", 
        # 0xAE: "ュ", 
        # 0xAF: "ョ", 
        # 0xB0: "ィ", 
        # 0xB1: "あ", $b1
        # 0xB2: "い", $b2
        # 0xB3: "う", $b3
        # 0xB4: "え", $b4
        # 0xB5: "お", $b5
        # 0xB6: "か", $b6
        # 0xB7: "き", $b7
        # 0xB8: "く", $b8
        # 0xB9: "け", $b9
        # 0xBA: "こ", $ba
        # 0xBB: "さ", $bb
        # 0xBC: "し", $bc
        # 0xBD: "す", $bd
        # 0xBE: "せ", $be
        # 0xBF: "そ", $bf
        # 0xC0: "た", $c0
        # 0xC1: "ち", $c1
        # 0xC2: "つ", $c2
        # 0xC3: "て", $c3
        # 0xC4: "と", $c4
        # 0xC5: "な", $c5
        # 0xC6: "に", $c6
        # 0xC7: "ぬ", $c7
        # 0xC8: "ね", $c8
        # 0xC9: "の", $c9
        # 0xCA: "は", $ca
        # 0xCB: "ひ", $cb
        # 0xCC: "ふ", $cc
        # 0xCD: "へ", $cd
        # 0xCE: "ほ", $ce
        # 0xCF: "ま", $cf
        # 0xD0: "み", $d0
        # 0xD1: "む", $d1
        # 0xD2: "め", $d2
        # 0xD3 "も", $d3
        # 0xD4: "や", $d4
        # 0xD5: "ゆ", $d5
        # 0xD6: "よ", $d6
        # 0xD7: "ら", $d7
        # 0xD8: "り", $d8
        # 0xD9: "る", $d9
        # 0xDA: "れ", $da
        # 0xDB: "ろ", $db
        # 0xDC: "わ", $dc
        # 0xDD: "を", $dd
        # 0xDE: "ん", $de
        # 0xDF: "っ", $df
        # 0xE0: "ゃ", $e0
        # 0xE1: "ゅ", $e1
        # 0xE2: "ょ", $e2
        # 0xE3: "ー", $e3
        # 0xE4: "ﾟ", $e4
        # 0xE5: "ﾞ", $e5
        # 0xE6: "？", $e6
        # 0xE7: "！", $e7
        # 0xE8: "。", $e8

        # 0xF0: "円", $f0

        # 0xF2: "．", $f2
        # 0xF3: "／", $f3

        # 0xF4: "ォ", $f4

        0xF0: "¥",
        0xF1: "×",
        0xF3: "/",
        0xF4: ",",
        0xF5: "♀",
        0xF6: "0",
        0xF7: "1",
        0xF8: "2",
        0xF9: "3",
        0xFA: "4",
        0xFB: "5",
        0xFC: "6",
        0xFD: "7",
        0xFE: "8",
        0xFF: "9"
    }

    def decodeText(self) -> str:
        return list(map(self.ALPHABET.get, self.packet.data))

    def __init__(self,packet) -> None:
        self.packet = packet
        self.text =  self.decodeText()
       

    def __str__(self):
        return "".join(self.text).strip('@')

    def __len__(self):
        return len(self.packet)

#Constants that have hard pointers in Red/Blue
ROM = ConstBitStream(filename='pokered.gbc')
BANK_SIZE = 0x4000
TWO_BPP_TILE_SIZE = 16
ONE_BPP_TILE_SIZE = 8
BYTE = 8
BIT = 1
NYBBLE = 4
TWO_BPP = 2
ONE_BPP = 1

POKEMON_NAME_LENGTH = 10

END_FILE = Addr.convert_to_addr(ROM.len/8)

POKEDEX_ORDER_POINTER = Addr(0x10,0x5024)
POKEDEX_ENTRY_POINTER = Addr(0x10,0x447e)
POKEMON_DATA_POINTER  = Addr(0X0E,0x43DE)
POKEMON_NAME_POINTER  = Addr(0x07,0x421e)
MOVE_NAME_POINTER     = Addr(0x2C,0x4000)
MOVES_DATA_POINTER    = Addr(0x0E,0x4000)
TM_HM_LIST_POINTER    = Addr(0x04,0x7773)
FONT_START_POINTER    = Addr(0x04,0x5a80)


datamap = {'Index to Pokedex':  [],
           'Pokedex Entry Loc': []
}

pokedex_index_map = []
pokedex_loc_map = []

for i in range(0,380,2):
    datamap["Pokedex Entry Loc"].append(GBDataPacket.get_static_data(POKEDEX_ENTRY_POINTER+i,BYTE,2).collapse(rev=True))
    datamap["Index to Pokedex"].append(GBDataPacket.get_static_data(POKEDEX_ORDER_POINTER+int(i/2),BYTE,1).collapse())