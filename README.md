# Poshmon
Poshmon is an ambitious work to port Pokémon Blue/Red version to native Powershell.
Doing this in my spare time to get the port working. If you want to try and help speed up production buy me more caffine.
[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/R6R75BZED)
## How to use
Some prework you might have to do is to ensure you can display 160 characters wide by 72 characters tall. On a 1080p monitor this translates to max size 14 font of Consolas. A good balance I have found is size 12 font and SimSun-ExtB. Any font will work as long as each character is excatly twice as tall as wide. If that ratio is off weird scaling issues can happen.

If you want to scrape your own ROM you should update the pointers in the [pokedata.py](https://github.com/super-phreak/poshmon/blob/master/poshmon-tools/pokedata.py).

There is a server and client compenent. The server is written in rust and is in a Pre-Alpha stage. The client is written in native Powershell 5 and is also in prealpha.

Currently I am closed to pull requests. If you want to help the best way is to buy me that coffee!
[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/R6R75BZED)

## Current Objectives
- [x] Python scripts to scrape Gameboy ROM binaries and gather the Pokémon
- [x] Graphics funtional
- [ ] Animation
- [ ] Mulitplayer
- [ ] Server runspace
- [ ] Containerized swarm
- [ ] Soul removed from body
- [x] Coffee drank