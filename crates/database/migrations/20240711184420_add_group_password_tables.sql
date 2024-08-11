BEGIN;

ALTER TABLE bloodmessages
ADD COLUMN group_passwords TEXT[] NOT NULL;

ALTER TABLE bloodstains
ADD COLUMN group_passwords TEXT[] NOT NULL;

ALTER TABLE ghostdata
ADD COLUMN group_passwords TEXT[] NOT NULL;

COMMIT;
