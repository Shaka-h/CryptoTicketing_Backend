-- Your SQL goes here

CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    userId INTEGER NOT NULL,
    eventName TEXT NOT NULL,
    eventDescription TEXT NOT NULL,
    eventDate DATE NOT NULL CHECK (eventDate >= CURRENT_DATE),
    eventDateTime TIMESTAMP NOT NULL,
    eventType VARCHAR(255) NOT NULL,
    eventCountry TEXT NOT NULL,
    eventCity TEXT NOT NULL,
    eventPlace TEXT NOT NULL,
    eventImage TEXT NOT NULL,
    eventTicketPrice INTEGER NOT NULL,
    eventLiked BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (userId) REFERENCES users(id) ON DELETE CASCADE
)
