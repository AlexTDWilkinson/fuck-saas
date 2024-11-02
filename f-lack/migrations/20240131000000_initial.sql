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
    name TEXT NOT NULL
    -- created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS message (
    -- created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    channel_id INTEGER NOT NULL references channel(id),
    message_index INTEGER NOT NULL,
    creator_id INTEGER NOT NULL references account(id),
    content TEXT NOT NULL,
    PRIMARY KEY (channel_id, message_index)
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

INSERT INTO channel (id,name)
     VALUES (0, 'general'),
            (1, 'random'),
	    (2, 'announcements');

INSERT INTO message (creator_id,channel_id,message_index,content)
     VALUES (1, 0, 0, 'I am alice! Hello general channel'),
            (2, 0, 1, 'Hi Alice! Welcome to general channel'),
	    (1, 0, 2, 'Thanks for welcoming me Bob!'),
            (2, 0, 3, 'No problem Alice, its literally my job. They will actually fire me.'),
	    (1, 0, 4, 'Oh, well then I retract my thanks.'),
            (2, 0, 5, '*sigh*, its a living.'),
            (1, 1, 0, 'Can I also post in the random channel?'),
            (2, 1, 1, 'Yeah, we cant stop you.'),
            (1, 1, 2, 'LOL! Thats funny'),
            (2, 1, 3, 'No literally, we have no security implement yet!');
