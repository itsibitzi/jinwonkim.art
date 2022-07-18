PRAGMA foreign_keys = ON;

CREATE TABLE users (
    username      TEXT NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE sessions (
    id      TEXT NOT NULL,
    expires TEXT NOT NULL -- ISO formatted date
);

CREATE TABLE categories (
    id   TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE images (
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name        TEXT NOT NULL,
    description TEXT NOT NULL,
    filename    TEXT NOT NULL
);

CREATE TABLE category_images (
    category_id TEXT REFERENCES categories(id) ON DELETE CASCADE NOT NULL,
    image_id    INTEGER REFERENCES images(id) ON DELETE CASCADE NOT NULL
);

CREATE TABLE about (
    id         TEXT PRIMARY KEY,
    about_text TEXT NOT NULL
);

CREATE TABLE faqs (
    id       INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    question TEXT NOT NULL,
    answer   TEXT NOT NULL
);