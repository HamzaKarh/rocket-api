-- Your SQL goes here
CREATE TABLE files (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  dir VARCHAR NOT NULL
--  body TEXT NOT NULL,
--  directory BOOLEAN NOT NULL DEFAULT 'f'
)