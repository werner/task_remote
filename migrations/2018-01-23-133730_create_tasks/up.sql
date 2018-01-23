CREATE TABLE tasks (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  pre_hook TEXT,
  code TEXT NOT NULL,
  post_hook TEXT,
  language TEXT
);