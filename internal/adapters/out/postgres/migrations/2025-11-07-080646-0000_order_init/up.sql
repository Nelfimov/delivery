-- Your SQL goes here
CREATE TABLE "orders"(
	"id" UUID NOT NULL PRIMARY KEY,
	"courier_id" UUID,
	"location_x" SMALLINT NOT NULL,
	"location_y" SMALLINT NOT NULL,
	"volume" SMALLINT NOT NULL,
	"status" TEXT NOT NULL
);

