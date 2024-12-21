-- Your SQL goes here
CREATE TABLE contacts
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name  TEXT NOT NULL,
    email      TEXT NOT NULL,
    address    TEXT NOT NULL,
    phone      TEXT NOT NULL
);