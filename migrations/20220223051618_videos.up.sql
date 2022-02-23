-- Add up migration script here
CREATE TABLE IF NOT EXISTS videos
(
    youtube_id      TEXT PRIMARY KEY NOT NULL,
    youtube_list_id TEXT NOT NULL,
    published       BOOLEAN NOT NULL DEFAULT FALSE
);
