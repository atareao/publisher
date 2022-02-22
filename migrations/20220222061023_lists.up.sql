-- Add up migration script here
CREATE TABLE IF NOT EXISTS lists
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    youtube_id TEXT NOT NULL,
    name       TEXT NOT NULL,
    reverse    INTEGER NOT NULL DEFAULT 0
);
