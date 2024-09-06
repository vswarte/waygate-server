CREATE TABLE ugc (
    ugc_id SERIAL PRIMARY KEY,
    ugc_code VARCHAR NOT NULL,
    steam_id VARCHAR NOT NULL,
    data BYTEA NOT NULL
);
