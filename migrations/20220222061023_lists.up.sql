-- Add up migration script here
CREATE TABLE IF NOT EXISTS lists
(
    youtube_id TEXT PRIMARY KEY NOT NULL,
    name       TEXT NOT NULL,
    reverse    BOOLEAN NOT NULL DEFAULT FALSE
);
