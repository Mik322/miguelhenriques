-- Your SQL goes here
CREATE TABLE projects (
    id SERIAL PRIMARY KEY NOT NULL,
    image_name VARCHAR,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL
)