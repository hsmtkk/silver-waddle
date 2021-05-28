-- Your SQL goes here
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL
);

INSERT INTO todos (content) VALUES ("alpha");
INSERT INTO todos (content) VALUES ("bravo");
