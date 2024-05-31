create extension if not exists "uuid-ossp";

-- Create "users" table
CREATE TABLE "users" (
  "id" uuid NOT NULL DEFAULT uuid_generate_v1mc(),
  "name" character varying NOT NULL,
  "email" character varying NOT NULL,
  "createdAt" timestamp NOT NULL,
  "updatedAt" timestamp NOT NULL,
  PRIMARY KEY ("id")
);
-- Create "task" table
CREATE TABLE "task" (
  "id" uuid NOT NULL,
  "title" character varying NOT NULL,
  "description" character varying NOT NULL,
  "done" boolean NOT NULL,
  "owner_id" uuid NOT NULL,
  "createdAt" timestamp NOT NULL,
  "updatedAt" timestamp NOT NULL,
  PRIMARY KEY ("id"),
  CONSTRAINT "author_fk" FOREIGN KEY ("owner_id") REFERENCES "users" ("id")
    ON UPDATE NO ACTION ON DELETE CASCADE
);
