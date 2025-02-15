ALTER TABLE sessions ALTER COLUMN session_id SET DATA TYPE bigint;
ALTER SEQUENCE sessions_session_id_seq AS bigint;

ALTER TABLE bloodmessages ALTER COLUMN bloodmessage_id SET DATA TYPE bigint;
ALTER TABLE bloodmessages ALTER COLUMN session_id SET DATA TYPE bigint;
ALTER SEQUENCE bloodmessages_bloodmessage_id_seq AS bigint;

ALTER TABLE ghostdata ALTER COLUMN ghostdata_id SET DATA TYPE bigint;
ALTER TABLE ghostdata ALTER COLUMN session_id SET DATA TYPE bigint;
ALTER SEQUENCE ghostdata_ghostdata_id_seq AS bigint;

ALTER TABLE bloodstains ALTER COLUMN bloodstain_id SET DATA TYPE bigint;
ALTER TABLE bloodstains ALTER COLUMN session_id SET DATA TYPE bigint;
ALTER SEQUENCE bloodstains_bloodstain_id_seq AS bigint;

ALTER TABLE player_equipments ALTER COLUMN player_equipments_id SET DATA TYPE bigint;
ALTER TABLE player_equipments ALTER COLUMN session_id SET DATA TYPE bigint;
ALTER SEQUENCE player_equipments_player_equipments_id_seq AS bigint;

ALTER TABLE bans ALTER COLUMN ban_id SET DATA TYPE bigint;
ALTER SEQUENCE bans_ban_id_seq AS bigint;
