# Waygate :cyclone: Elden Ring Private Server
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

## Setup
### Server setup
The server requires a postgresql database to store messages, bloodstains, ghosts
and more. I am not going to cover how to set up a postgresql database as enough
places cover this exact procedure.

### Keys
The game uses a set of fixed keys to kick off the connection and perform the key
exchange. You will need to generate a key pair for your server and client. You
do this once during setup of the server.

In order to generate a keypair you invoke:
```bash
$ ./waygate-generate-keys
```

This will print a valid keypair alongside instructions on applying the keypair
to your setup:
```
# Pass this to your servers launch command
--client-public-key = "8oWXtzzyvMwg0DZTUxdRP/HzDnDhlw8J1ZyXiB2Giks="
--server-secret-key = "XzUteM9hPf2n/XUg8L2ImxIaRUGusUpqYVFDnEY0Egs="

# Add this to your clients's waygate.toml
client_secret_key = "VD7xDTwd6+kt9zg+3qzVaKnxfIvVIwSG8JM1cc8Eeu0="
server_public_key = "5HCUh3iOJiPwsEPvln0QFnmx9sFaQrzDI9XopTa532c="
```

### Running the server
Grab the [latest release](https://github.com/vswarte/waygate-server/releases)
for your platform and invoke it as such:

```bash
$ ./waygate-server \
    --bind 0.0.0.0:10901 \
    --api-bind 0.0.0.0:10902 \
    --api-key <API KEY> \
    --database "<DATABASE URL>" \
    --client-public-key "<CLIENT PUBLIC KEY>" \
    --server-secret-key "<SERVER SECRET KEY>"
```

| Parameter             | Env variable                | Description                                        |
|-----------------------|-----------------------------|----------------------------------------------------|
| `--bind`              | `WAYGATE_BIND`              | Specifies the binding address for the game server. |
| `--api-bind`          | `WAYGATE_API_BIND`          | Specifies the binding address for the api server.  |
| `--api-key`           | `WAYGATE_API_KEY`           | Specifies API authentication key. Keep secret.     |
| `--database`          | `WAYGATE_DATABASE`          | Specifies the database URL to be used              |
| `--client-public-key` | `WAYGATE_CLIENT_PUBLIC_KEY` | Specifies the KX client public key. Keep secret.   |
| `--server-secret-key` | `WAYGATE_SERVER_SECRET_KEY` | Specifies the KX server secret key. Keep secret.   |

#### Database URL
The `--database` parameter expects a database URL like so: `postgresql://<USERNAME>:<PASSWORD>@<HOST>/<DATABASE>`.

#### Setting up the client
// TBD

### Additional configuration
#### Logging
The logging setup is configured with `logging.toml`. Under the hood it's log4rs,
which has its [own manual](https://docs.rs/log4rs/latest/log4rs/config/index.html) on the logging options.

#### Announcements
The announcements are set in `announcements.toml`.

### API
The server also spins up a JSON HTTP API that allows people to do automated healthchecks,
broadcast messages and more in the future. This HTTP server is bound seperately
from the game server dictated by the `--api-bind` parameter.

API authentication is regulated by the `--api-key` which requires you to specify
a key that must be matched on incoming HTTP requests. You can use random.org or
a password generator to derive a secure API key.

Example healtcheck call:
```bash
$ curl -X GET http://localhost:10902/health \
    --header "X-Auth-Token: <API KEY>" \
    --header "Content-Type: application/json"         
```

Example announcement call:
```bash
$ curl -v -X POST http://localhost:10902/notify/message \
    --header "X-Auth-Token: <API KEY>" \
    --header "Content-Type: application/json" \
    --data '{"message":"Test Announcement"}'
```

## What's working? What needs to be done?
 - [x] Summoning per sign
 - [x] Quickmatches (arena)
 - [x] Invasions
 - [x] Player messages
 - [x] Bloodstains
 - [x] Player ghosts
 - [x] Summoning per puddle
 - [x] Fia/Warrior Jar Pool
 - [x] Group passwords
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

# Credits
The most important section of this README, this project took a long time to
execute and there's still a bit to go. Without these people this project
would've remained on my disk forever.

 - Tremwil for implementing the serde wire (de)serializer and writing tools to hook the games serialization layer and generating typemaps from said hooks.
 - LukeYui for answering my dumb questions
 - ClayAmore for testing and support on the motivational end
 - Steelovsky for testing
 - Mintal for testing
 - Metalcrow for testing
 - Shion for testing
 - Dasaav for testing
 - auramalexander (Dylan) for name
