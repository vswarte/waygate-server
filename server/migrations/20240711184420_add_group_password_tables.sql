BEGIN;

ALTER TABLE bloodmessages
ADD COLUMN group_passwords TEXT[];

ALTER TABLE bloodstains
ADD COLUMN group_passwords TEXT[];

ALTER TABLE ghostdata
ADD COLUMN group_passwords TEXT[];

COMMIT;
