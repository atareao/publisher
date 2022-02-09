-- Your SQL goes here
CREATE TABLE days(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);
CREATE TABLE lists(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    youtube_id TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE,
    reverse BOOLEAN NOT NULL DEFAULT 'f'
);
CREATE TABLE videos(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    list_id INTEGER NOT NULL,
    youtube_id TEXT NOT NULL UNIQUE,
    published BOOLEAN NOT NULL DEFAULT 'f'
);
CREATE TABLE day_list(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    day_id INTEGER NOT NULL,
    list_id INTEGER NOT NULL,
    norder INTEGER NOT NULL
);