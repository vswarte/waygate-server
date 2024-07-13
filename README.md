# Waygate Elden Ring Private Server
Waygate is an attempt at implementing a server based on Elden Ring's matchmaking
protocol. The aim of this project is to offer a more vanilla online experience
while allowing users to mod the game.

Waygate is **not** an overhaul, aside from replacing the game's peer-to-peer
to be more bearable this mod does nothing to enhance the online experience by
itself.
I count on other mods to implement tweaks to the online experience, please do
not open feature suggestions pitching me a tweak that "would objectively
improve X".

### WARNING
Over the past year testing with about 30-40 people no ones has reported getting
banned from official online play, however there is no guarantee this server
is completely safe to use.

## What's working? What needs to be done?
 - [x] Summoning per sign
 - [x] Quickmatches (arena)
 - [x] Invasions
 - [x] Player messages
 - [x] Bloodstains
 - [x] Player ghosts
 - [x] Summoning per puddle
 - [x] Fia/Warrior Jar Pool
 - [x] Group passwords (WIP)
 - [ ] Blue Cipher Ring
 - [ ] Quickmatch ranking
 - [ ] Match Density (PvP activity on map)
 - [ ] A fuckton of telemetry-related messaging

### Non-gameplay tasks
#### Banning players
There is currently no way of banning players from your server. Blocking
people on steam works to keep them you from matching with these players.
A system for banning players from the server is being worked on.

#### Code cleanup and tidying
A lot of the development happened as I was reversing the game, so a few parts
need rewriting to be more maintainable. The database needs indices as an
optimization, etc

## Installation
The server requires a postgresql database to store messages, bloodstains, ghosts
and more. I am not going to cover how to set up a postgresql database as enough
places cover this exact procedure.

// TBD

# Credits
The most important section of this README, this project took a long time to
execute and there's still a bit to go. Without these people this project
would've remained on my disk forever.

 - Tremwil for implementing the serde wire (de)serializer and writing tools to hook the games serialization layer and typemaps.
 - LukeYui for answering my dumb questions
 - ClayAmore for testing and support on the motivational end
 - Steelovsky for testing
 - Mintal for testing
 - Metalcrow for testing
 - Dasaav for testing
 - Shion for testing
