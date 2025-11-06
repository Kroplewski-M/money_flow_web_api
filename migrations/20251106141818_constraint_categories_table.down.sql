-- Add down migration script here
ALTER TABLE categories DROP CONSTRAINT IF EXISTS title_not_empty;
