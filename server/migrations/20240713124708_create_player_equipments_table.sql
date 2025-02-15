CREATE TABLE player_equipments (
    player_equipments_id SERIAL PRIMARY KEY,
    player_id INTEGER NOT NULL,
    session_id INTEGER NOT NULL,
    data BYTEA NOT NULL,
    pool INTEGER NOT NULL
);
