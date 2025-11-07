-- Your SQL goes here
CREATE TABLE "storage_places"(
	"id" UUID NOT NULL PRIMARY KEY,
	"courier_id" UUID NOT NULL,
	"name" TEXT NOT NULL,
	"total_volume" SMALLINT NOT NULL,
	"order_id" UUID
);

