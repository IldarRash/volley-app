-- backend/migrations/20231027120002_create_events_table.down.sql

DROP TABLE IF EXISTS events;
DROP TYPE IF EXISTS event_type;
DROP TYPE IF EXISTS event_level;
DROP TYPE IF EXISTS participant_status;
DROP TYPE IF EXISTS payment_status; 