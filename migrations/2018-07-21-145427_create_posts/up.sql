-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  account VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  img_url_1 TEXT,
  img_url_2 TEXT,
  img_url_3 TEXT,
  img_url_4 TEXT,
  regulation BOOLEAN NOT NULL DEFAULT 'f'
);
CREATE TABLE post_img (
  id SERIAL PRIMARY KEY,
  account VARCHAR NOT NULL,
  img_url_1 TEXT,
  regulation BOOLEAN NOT NULL DEFAULT 'f'
);
