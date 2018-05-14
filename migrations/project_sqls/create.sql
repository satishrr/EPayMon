-- Your SQL goes here

CREATE TABLE questions (
  id SERIAL PRIMARY KEY,
  user_name TEXT NOT NULL,
  wallet_info VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL,
  user_id VARCHAR NOT NULL
)