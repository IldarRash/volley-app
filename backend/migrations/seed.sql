-- backend/migrations/seed.sql

-- Please replace 'hashed_password_admin' and 'hashed_password_user' with actual bcrypt hashes.
-- You can generate them using a tool or a script.

-- Users
INSERT INTO users (id, username, password_hash, role, rating, telegram_id, subscribed) VALUES
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'admin', 'hashed_password_admin', 'admin', 5.0, '123456789', true),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'user', 'hashed_password_user', 'user', 4.5, '987654321', false);

-- Locations
INSERT INTO locations (id, name, coordinates, confirmed, image_url) VALUES
('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'Beach Volley Central', '{"lat": 59.93428, "lon": 30.3351}', true, 'https://example.com/beach_volley.jpg'),
('b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'City Park Arena', '{"lat": 59.931, "lon": 30.332}', false, 'https://example.com/city_park.jpg');

-- Events
INSERT INTO events (id, name, description, event_type, location_id, datetime, level, price, trainer_id, confirmed, max_participants) VALUES
('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'Evening Training', 'Intermediate level training', 'training', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', NOW() + interval '1 day', 'intermediate', 10, 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', true, 16),
('c0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', 'Weekend Tournament', 'Open tournament for all levels', 'tournament', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a12', NOW() + interval '3 day', 'advanced', 25, NULL, true, 32);

-- Event Timers
INSERT INTO event_timers (id, name, location_id, event_type, level, day_of_week, "time", active, price, max_participants) VALUES
('d0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'Regular evening game', 'b0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'game', 'intermediate', 5, '19:00:00', true, 15, 12); 