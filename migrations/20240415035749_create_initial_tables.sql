CREATE TABLE players (
    player_id SERIAL PRIMARY KEY,
    external_id varchar NOT NULL
);

CREATE TABLE sessions (
    session_id SERIAL PRIMARY KEY,
    player_id INTEGER NOT NULL,
    cookie BYTEA NOT NULL
);

CREATE TABLE bloodmessages (
    bloodmessage_id SERIAL PRIMARY KEY,
    player_id INTEGER NOT NULL,
    character_id INTEGER NOT NULL,
    session_id INTEGER NOT NULL,
    rating_good INTEGER NOT NULL,
    rating_bad INTEGER NOT NULL,
    data BYTEA NOT NULL,
    area INTEGER NOT NULL,
    play_region INTEGER NOT NULL
);

CREATE TABLE ghostdata (
    ghostdata_id SERIAL PRIMARY KEY,
    player_id INTEGER NOT NULL,
    session_id INTEGER NOT NULL,
    replay_data BYTEA NOT NULL,
    area INTEGER NOT NULL,
    play_region INTEGER NOT NULL
);

CREATE TABLE bloodstains (
    bloodstain_id SERIAL PRIMARY KEY,
    player_id INTEGER NOT NULL,
    session_id INTEGER NOT NULL,
    advertisement_data BYTEA NOT NULL,
    replay_data BYTEA NOT NULL,
    area INTEGER NOT NULL,
    play_region INTEGER NOT NULL
);
