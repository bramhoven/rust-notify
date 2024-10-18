-- Your SQL goes here

CREATE TABLE "notifications"(
	"id" UUID NOT NULL DEFAULT uuid_generate_v4() PRIMARY KEY,
  "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
	"title" VARCHAR(100) NOT NULL,
	"body" VARCHAR(500) NOT NULL
);

