-- Your SQL goes here
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    image_name VARCHAR,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL
)