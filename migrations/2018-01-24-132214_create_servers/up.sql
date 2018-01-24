CREATE TABLE servers (
  id INTEGER PRIMARY KEY NOT NULL,
  user TEXT NOT NULL,
  domain_name TEXT NOT NULL,
  CHECK (user <> ""),
  CHECK (domain_name <> "")
);