-- backend/migrations/20231027120001_create_locations_table.up.sql

CREATE TABLE locations (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    coordinates JSONB NOT NULL,
    confirmed BOOLEAN NOT NULL DEFAULT false,
    image_url VARCHAR(255)
); 