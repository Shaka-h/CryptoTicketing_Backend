-- Your SQL goes here
CREATE TABLE likes (
    user_id SERIAL PRIMARY KEY REFERENCES users(id),
    event_id INTEGER NOT NULL REFERENCES events(id)
);


