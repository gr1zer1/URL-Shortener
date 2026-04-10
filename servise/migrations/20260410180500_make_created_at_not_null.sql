-- Add migration script here
ALTER TABLE links
ALTER COLUMN created_at SET DEFAULT NOW();

UPDATE links
SET created_at = NOW()
WHERE created_at IS NULL;

ALTER TABLE links
ALTER COLUMN created_at SET NOT NULL;
