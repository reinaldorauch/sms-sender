-- Your SQL goes here
CREATE TABLE "public"."user" (
  "user_id" SERIAL PRIMARY KEY,
  "email" VARCHAR NOT NULL,
  "username" VARCHAR NOT NULL,
  "password_hash" VARCHAR NOT NULL,
  "user_type" CHAR(1) NOT NULL -- S => User Sender, A => User Admin
)