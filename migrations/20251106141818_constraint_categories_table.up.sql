-- Add up migration script here
ALTER TABLE categories
ADD constraint title_not_empty CHECK (char_length(trim(title)) > 0);
