-- backend/migrations/20231027120003_create_event_timers_table.up.sql

CREATE TABLE event_timers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    location_id UUID REFERENCES locations(id),
    event_type event_type NOT NULL,
    level event_level,
    day_of_week INTEGER NOT NULL,
    "time" TIME NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    price INTEGER,
    max_participants INTEGER
); 