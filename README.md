# Poshmon
Poshmon is an ambitious work to port Pokémon Blue/Red version to native Powershell.
## How to use
If you want to scrape your own ROM you should update the pointers in the [pokedata.py](https://github.com/super-phreak/poshmon/blob/master/poshmon-tools/pokedata.py).

If you want to run the Powershell command to test the Pokédex you must search by either:
* `-Name`
* `-PokedexIndex`
* `-InternalIndex`

You cannot combine the switches. There is also a `-Flip` switch to mirror the Sprite horizontally

## Current Objectives
- [x] Python scripts to scrape Gameboy ROM binaries and gather the Pokémon
- [x] Fonts is also there
- [x] Powershell rudimentary graphics library is also functional
- [x] Display the Pokédex entry in native font from the ROM
- [ ] Get keypress working
- [ ] Add Pokedex scroll animation
- [ ] Get game running
