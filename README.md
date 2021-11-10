# Poshmon
Poshmon is an ambitious work to port Pokémon Blue/Red version to native Powershell.
## How to use
Some prework you might have to do is to ensure you can display 160 characters wide by 72 characters tall. On a 1080p monitor this translates to max size 14 font of Consolas. A good balance I have found is size 12 font and SimSun-ExtB. Any font will work as long as each character is excatly twice as tall as wide. If that ratio is off weird scaling issues can happen.

If you want to scrape your own ROM you should update the pointers in the [pokedata.py](https://github.com/super-phreak/poshmon/blob/master/poshmon-tools/pokedata.py).

If you want to run the Powershell command to test the Pokédex you must search by either:
* `-Name`
* `-PokedexIndex`
* `-InternalIndex`
* `-Random`

You can add `-Scroll` and `-ScrollLength` to Auto-scroll through the Pokedex. E.X. `Show-Pokedex -Random -Scroll 5` will show 6 (The orginal Pokémon + 5 moves).
You can do infinite scrolling by passing `-Scroll -1`.

## Current Objectives
- [x] Python scripts to scrape Gameboy ROM binaries and gather the Pokémon
- [x] Fonts is also there
- [x] Powershell rudimentary graphics library is also functional
- [x] Display the Pokédex entry in native font from the ROM
- [x] Get keypress working
- [x] Add Pokedex scroll animation
- [ ] Get game running
