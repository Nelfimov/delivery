-- Your SQL goes here
CREATE TABLE "outbox"(
	"id" UUID NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"payload" TEXT NOT NULL,
	"occured_at" TIMESTAMP NOT NULL,
	"processed_at" TIMESTAMP
);

