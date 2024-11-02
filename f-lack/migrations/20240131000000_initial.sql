CREATE TABLE IF NOT EXISTS account (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    permissions TEXT NOT NULL,
    set_password_mode BOOLEAN NOT NULL DEFAULT 0,
    set_password_pin INTEGER NOT NULL DEFAULT 0,
    set_password_attempts INTEGER NOT NULL DEFAULT 0,
    user_disabled BOOLEAN NOT NULL DEFAULT 0,
    user_deleted BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS channel (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS session (
    user_id INTEGER NOT NULL REFERENCES account(id),
    valid_until TIMESTAMP NOT NULL
);