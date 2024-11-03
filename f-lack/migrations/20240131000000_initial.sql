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
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS message (
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    channel_id INTEGER NOT NULL references channel(id),
    creator_id INTEGER NOT NULL references account(id),
    content TEXT NOT NULL,
    PRIMARY KEY (channel_id, created_at)
);
CREATE TABLE IF NOT EXISTS session (
    user_id INTEGER NOT NULL REFERENCES account(id),
    valid_until TIMESTAMP NOT NULL
);

INSERT INTO account (id,username,email,password_hash,permissions)
     VALUES (0, 'test_admin', 'test_email', 'blahblah', 'admin'),
            (1, 'alice', 'alice_email', 'blahblah', ''),
            (2, 'bob', 'bob_email', 'blahblah', ''),
            (3, 'charlie', 'charlie_email', 'blahblah', '');

INSERT INTO channel (id,name,created_at)
     VALUES (0, 'General', '2024-01-31 09:00:00'),
            (1, 'Random', '2024-01-31 09:00:00'),
            (2, 'Announcements', '2024-01-31 09:00:00');

INSERT INTO message (created_at,creator_id,channel_id,content)
     VALUES ('2024-01-31 10:00:00', 1, 0, 'I am alice! Hello general channel'),
            ('2024-01-31 10:01:00', 2, 0, 'Hi Alice! Welcome to general channel'),
	        ('2024-01-31 10:02:00', 1, 0, 'Thanks for welcoming me Bob!'),
            ('2024-01-31 10:03:00', 2, 0,  'No problem Alice, its literally my job. They will actually fire me.'),
	        ('2024-01-31 10:04:00', 1, 0,  'Oh, well then I retract my thanks.'),
            ('2024-01-31 10:05:00', 2, 0, '*sigh*, its a living.'),
            ('2024-01-31 10:06:00', 1, 1,  'Can I also post in the random channel?'),
            ('2024-01-31 10:07:00', 2, 1,  'Yeah, we cant stop you.'),
            ('2024-01-31 10:08:00', 1, 1,  'LOL! Thats funny'),
            ('2024-01-31 10:09:00', 2, 1,  'No literally, we have no security implement yet!');
