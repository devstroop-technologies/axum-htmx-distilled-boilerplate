-- Items table — matches the existing Item struct
CREATE TABLE IF NOT EXISTS items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    done INTEGER NOT NULL DEFAULT 0
);

-- Seed with the same example data the in-memory service uses
INSERT INTO items (title, description, done) VALUES
    ('Set up project', 'Scaffold Axum + HTMX boilerplate', 1),
    ('Add database', 'Integrate SQLite or Postgres', 0),
    ('Deploy', 'Containerize and ship to production', 0);
