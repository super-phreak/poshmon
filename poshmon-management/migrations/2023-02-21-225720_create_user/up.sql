-- Your SQL goes here
CREATE TABLE users (
  id UUID PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  hash TEXT NOT NULL
);