-- Add up migration script here
CREATE TABLE IF NOT EXISTS emails(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    from INTEGER NOT NULL,
    to TEXT NOT NULL,
    subject TEXT,
    body TEXT,
    date DATETIME,
    inbox BOOLEAN
);
