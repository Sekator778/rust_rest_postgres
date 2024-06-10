-- Your SQL goes here
CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       name VARCHAR NOT NULL,
                       email VARCHAR NOT NULL UNIQUE
);
