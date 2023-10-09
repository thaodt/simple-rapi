-- Up
CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT
);

-- Down
DROP TABLE items;
