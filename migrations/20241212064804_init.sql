-- Add migration script here
-- migrations/<timestamp>_create_users_table.sql

CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);
