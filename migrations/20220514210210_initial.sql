PRAGMA foreign_keys = ON;

CREATE TABLE users (
    username      TEXT PRIMARY KEY NOT NULL,
    password_hash TEXT NOT NULL
);

CREATE TABLE categories (
    id   TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE images (
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    position    INTEGER,
    name        TEXT NOT NULL,
    description TEXT NOT NULL,
    filename    TEXT NOT NULL
);

CREATE TRIGGER auto_increment_image_position
    AFTER INSERT ON images
    WHEN new.position IS NULL
    BEGIN
        UPDATE images
        SET position = (SELECT IFNULL(MAX(position), 0) + 1 FROM images)
        WHERE id = new.id;
    END;

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
    position INTEGER,
    question TEXT NOT NULL,
    answer   TEXT NOT NULL
);


CREATE TRIGGER auto_increment_faq_position
    AFTER INSERT ON faqs
    WHEN new.position IS NULL
    BEGIN
        UPDATE faqs
        SET position = (SELECT IFNULL(MAX(position), 0) + 1 FROM faqs)
        WHERE id = new.id;
    END;