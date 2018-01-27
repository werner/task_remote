CREATE TABLE languages (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  value TEXT NOT NULL,
  CHECK (name <> ""),
  CHECK (value <> "")
);

INSERT INTO languages(name, value) VALUES ("Ruby", "ruby");
INSERT INTO languages(name, value) VALUES ("Python", "python");
INSERT INTO languages(name, value) VALUES ("Perl", "perl");
INSERT INTO languages(name, value) VALUES ("Javascript", "js");
INSERT INTO languages(name, value) VALUES ("PHP", "php");