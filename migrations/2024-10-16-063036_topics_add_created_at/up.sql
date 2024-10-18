-- Your SQL goes here

ALTER TABLE "topics" ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW();
