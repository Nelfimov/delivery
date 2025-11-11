-- Your SQL goes here
CREATE TABLE "couriers"(
	"id" UUID NOT NULL PRIMARY KEY,
	"name" TEXT NOT NULL,
	"speed" SMALLINT NOT NULL,
	"location_x" SMALLINT NOT NULL,
	"location_y" SMALLINT NOT NULL
);

