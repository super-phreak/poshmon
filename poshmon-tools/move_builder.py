from pokedata import GBDataPacket as data, GBText as text, Addr
from move import Move

import move
import pokedata 
import json

moves = []
start_addr = Addr(0xE,0x4000)

for i in range(165):
    moves.append(Move.from_addr(addr=start_addr+(i*move.MOVE_DATA_LENGTH)).to_json())

with open('../data/movedex.json', 'w') as moves_file:
    json.dump(moves, moves_file, indent=2)

##Pulled effect names from the decompiled asm
effects = {
    0x00: "NO_ADDITIONAL_EFFECT"      , # Plain Attack
	0x01: "EFFECT_01"                 ,  # unused
	0x02: "POISON_SIDE_EFFECT1"       ,  #
	0x03: "DRAIN_HP_EFFECT"           ,  #
	0x04: "BURN_SIDE_EFFECT1"         ,  #
	0x05: "FREEZE_SIDE_EFFECT"        ,  #
	0x06: "PARALYZE_SIDE_EFFECT1"     ,  #
	0x07: "EXPLODE_EFFECT"            ,  # Explosion, Self Destruct
	0x08: "DREAM_EATER_EFFECT"        ,  #
	0x09: "MIRROR_MOVE_EFFECT"        ,  #
	0x0A: "ATTACK_UP1_EFFECT"         ,  #
	0x0B: "DEFENSE_UP1_EFFECT"        ,  #
	0x0C: "SPEED_UP1_EFFECT"          ,  #
	0x0D: "SPECIAL_UP1_EFFECT"        ,  #
	0x0E: "ACCURACY_UP1_EFFECT"       ,  #
	0x0F: "EVASION_UP1_EFFECT"        ,  #
	0x10: "PAY_DAY_EFFECT"            ,  #
	0x11: "SWIFT_EFFECT"              ,  #
	0x12: "ATTACK_DOWN1_EFFECT"       ,  #
	0x13: "DEFENSE_DOWN1_EFFECT"      ,  #
	0x14: "SPEED_DOWN1_EFFECT"        ,  #
	0x15: "SPECIAL_DOWN1_EFFECT"      ,  #
	0x16: "ACCURACY_DOWN1_EFFECT"     ,  #
	0x17: "EVASION_DOWN1_EFFECT"      ,  #
	0x18: "CONVERSION_EFFECT"         ,  #
	0x19: "HAZE_EFFECT"               ,  #
	0x1A: "BIDE_EFFECT"               ,  #
	0x1B: "THRASH_PETAL_DANCE_EFFECT" ,  #
	0x1C: "SWITCH_AND_TELEPORT_EFFECT",  #
	0x1D: "TWO_TO_FIVE_ATTACKS_EFFECT",  #
	0x1E: "EFFECT_1E"                 ,  # unused
	0x1F: "FLINCH_SIDE_EFFECT1"       ,  #
	0x20: "SLEEP_EFFECT"              ,  #
	0x21: "POISON_SIDE_EFFECT2"       ,  #
	0x22: "BURN_SIDE_EFFECT2"         ,  #
	0x24: "PARALYZE_SIDE_EFFECT2"     ,  #
	0x25: "FLINCH_SIDE_EFFECT2"       ,  #
	0x26: "OHKO_EFFECT"               ,  # moves like Horn Drill
	0x27: "CHARGE_EFFECT"             ,  # moves like Solar Beam
	0x28: "SUPER_FANG_EFFECT"         ,  #
	0x29: "SPECIAL_DAMAGE_EFFECT"     ,  # Seismic Toss, Night Shade, Sonic Boom, Dragon Rage, Psywave
	0x2A: "TRAPPING_EFFECT"           ,  # moves like Wrap
	0x2B: "FLY_EFFECT"                ,  #
	0x2C: "ATTACK_TWICE_EFFECT"       ,  #
	0x2D: "JUMP_KICK_EFFECT"          ,  # Jump Kick and Hi Jump Kick effect
	0x2E: "MIST_EFFECT"               ,  #
	0x2F: "FOCUS_ENERGY_EFFECT"       ,  #
	0x30: "RECOIL_EFFECT"             ,  # moves like Double Edge
	0x31: "CONFUSION_EFFECT"          ,  # Confuse Ray, Supersonic (not the move Confusion)
	0x32: "ATTACK_UP2_EFFECT"         ,  #
	0x33: "DEFENSE_UP2_EFFECT"        ,  #
	0x34: "SPEED_UP2_EFFECT"          ,  #
	0x35: "SPECIAL_UP2_EFFECT"        ,  #
	0x36: "ACCURACY_UP2_EFFECT"       ,  #
	0x37: "EVASION_UP2_EFFECT"        ,  #
	0x38: "HEAL_EFFECT"               ,  # Recover, Softboiled, Rest
	0x39: "TRANSFORM_EFFECT"          ,  #
	0x3A: "ATTACK_DOWN2_EFFECT"       ,  #
	0x3B: "DEFENSE_DOWN2_EFFECT"      ,  #
	0x3C: "SPEED_DOWN2_EFFECT"        ,  #
	0x3D: "SPECIAL_DOWN2_EFFECT"      ,  #
	0x3E: "ACCURACY_DOWN2_EFFECT"     ,  #
	0x3F: "EVASION_DOWN2_EFFECT"      ,  #
	0x40: "LIGHT_SCREEN_EFFECT"       ,  #
	0x41: "REFLECT_EFFECT"            ,  #
	0x42: "POISON_EFFECT"             ,  #
	0x43: "PARALYZE_EFFECT"           ,  #
	0x44: "ATTACK_DOWN_SIDE_EFFECT"   ,  #
	0x45: "DEFENSE_DOWN_SIDE_EFFECT"  ,  #
	0x46: "SPEED_DOWN_SIDE_EFFECT"    ,  #
	0x47: "SPECIAL_DOWN_SIDE_EFFECT"  ,  #
	0x4C: "CONFUSION_SIDE_EFFECT"     ,  #
	0x4D: "TWINEEDLE_EFFECT"          ,  #
	0x4F: "SUBSTITUTE_EFFECT"         ,  #
	0x50: "HYPER_BEAM_EFFECT"         ,  #
	0x51: "RAGE_EFFECT"               ,  #
	0x52: "MIMIC_EFFECT"              ,  #
	0x53: "METRONOME_EFFECT"          ,  #
	0x54: "LEECH_SEED_EFFECT"         ,  #
	0x55: "SPLASH_EFFECT"             ,  #
	0x56: "DISABLE_EFFECT"               #
}

with open('../data/effectdex.json', 'w') as effects_file:
    json.dump(effects, effects_file, indent=2)