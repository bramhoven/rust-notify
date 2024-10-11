-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "topics"(
	"id" UUID NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
	"name" VARCHAR NOT NULL UNIQUE
);

