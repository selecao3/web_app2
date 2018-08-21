-- Your SQL goes here
CREATE TABLE post_img (
  id SERIAL PRIMARY KEY,
  account VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  img_url_1 TEXT,
  img_url_2 TEXT,
  img_url_3 TEXT,
  img_url_4 TEXT,
  regulation BOOLEAN NOT NULL DEFAULT 'f',
  created_day VARCHAR
);
-- userは予約語なので不可
CREATE TABLE profile (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  account VARCHAR NOT NULL,
  profile_text TEXT NOT NULL,
  profile_img TEXT NOT NULL,
  regulation BOOLEAN NOT NULL DEFAULT 'f',
  created_day VARCHAR
);

CREATE TABLE creater(
  id SERIAL PRIMARY KEY,
  account VARCHAR NOT NULL,
  mail_address VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  regulation BOOLEAN NOT NULL DEFAULT 'f',
  UNIQUE (account, mail_address)
);
