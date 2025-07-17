-- backend/migrations/20231027120002_create_events_table.up.sql

CREATE TYPE event_type AS ENUM ('training', 'game', 'tournament');
CREATE TYPE event_level AS ENUM ('beginner', 'intermediate', 'advanced', 'pro');
CREATE TYPE participant_status AS ENUM ('pending', 'confirmed');
CREATE TYPE payment_status AS ENUM ('manual', 'paid', 'unpaid');

CREATE TABLE events (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    event_type event_type NOT NULL,
    location_id UUID REFERENCES locations(id),
    datetime TIMESTAMPTZ NOT NULL,
    level event_level,
    price INTEGER,
    trainer_id UUID REFERENCES users(id),
    confirmed BOOLEAN NOT NULL DEFAULT false,
    participants JSONB NOT NULL DEFAULT '[]',
    max_participants INTEGER
); 