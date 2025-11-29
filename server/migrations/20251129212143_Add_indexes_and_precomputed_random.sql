ALTER TABLE ghostdata ADD COLUMN IF NOT EXISTS rnd double precision;
ALTER TABLE bloodmessages ADD COLUMN IF NOT EXISTS rnd double precision;
ALTER TABLE bloodstains ADD COLUMN IF NOT EXISTS rnd double precision;
ALTER TABLE player_equipments ADD COLUMN IF NOT EXISTS rnd double precision;

UPDATE ghostdata SET rnd = random();
UPDATE bloodmessages SET rnd = random();
UPDATE bloodstains SET rnd = random();
UPDATE player_equipments SET rnd = random();

ALTER TABLE ghostdata ALTER COLUMN rnd SET NOT NULL, ALTER COLUMN rnd SET DEFAULT random();
ALTER TABLE bloodmessages ALTER COLUMN rnd SET NOT NULL, ALTER COLUMN rnd SET DEFAULT random();
ALTER TABLE bloodstains ALTER COLUMN rnd SET NOT NULL, ALTER COLUMN rnd SET DEFAULT random();
ALTER TABLE player_equipments ALTER COLUMN rnd SET NOT NULL, ALTER COLUMN rnd SET DEFAULT random();

CREATE INDEX IF NOT EXISTS idx_ghostdata_play_region_rnd ON ghostdata (play_region, rnd);
CREATE INDEX IF NOT EXISTS idx_bloodmessages_play_region_rnd ON bloodmessages (play_region, rnd);
CREATE INDEX IF NOT EXISTS idx_bloodstains_play_region_rnd ON bloodstains (play_region, rnd);
CREATE INDEX IF NOT EXISTS idx_player_equipments_pool_rnd ON player_equipments (pool, rnd);

-- drop rows that violate the unique constraint
DELETE FROM player_equipments pe
WHERE EXISTS (
    SELECT 1 FROM player_equipments pe2
    WHERE pe2.pool = pe.pool
      AND pe2.player_id = pe.player_id
      AND pe2.player_equipments_id > pe.player_equipments_id
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_unique_player_equipments_pool_player ON player_equipments (pool, player_id);

CREATE INDEX IF NOT EXISTS idx_bans_external_id ON bans (external_id);
CREATE INDEX IF NOT EXISTS idx_players_external_id ON players (external_id);
