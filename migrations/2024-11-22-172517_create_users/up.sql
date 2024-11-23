-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  image TEXT,
  hash TEXT NOT NULL
);

