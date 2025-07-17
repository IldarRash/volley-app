-- backend/migrations/20231027120000_create_users_table.up.sql

CREATE TYPE user_role AS ENUM ('admin', 'user');

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL,
    rating DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    telegram_id VARCHAR(255) UNIQUE,
    instagram_link VARCHAR(255),
    image_url VARCHAR(255),
    subscriptions JSONB NOT NULL DEFAULT '[]',
    subscribed BOOLEAN NOT NULL DEFAULT false
); 