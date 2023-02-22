-- Your SQL goes here
CREATE TABLE users (
  id UUID NOT NULL,
  username TEXT NOT NULL,
  hash TEXT NOT NULL,
  PRIMARY KEY (id, username)
)