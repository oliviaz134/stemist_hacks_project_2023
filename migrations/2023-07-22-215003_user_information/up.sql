-- Your SQL goes here
CREATE TABLE user_information (
  user_id bigint PRIMARY KEY,
  username TEXT NOT NULL,
  points bigint NOT NULL
);

UPDATE user_information SET points = 0;
