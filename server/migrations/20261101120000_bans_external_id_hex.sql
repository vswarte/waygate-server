UPDATE bans
SET external_id = lower(to_hex(external_id::bigint))
WHERE external_id ~ '^7656119[0-9]{10}$';
