CREATE TABLE IF NOT EXISTS account (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
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
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS message (
    created_at INTEGER NOT NULL,
    channel_id INTEGER NOT NULL references channel(id),
    creator_id INTEGER NOT NULL references account(id),
    content TEXT NOT NULL,
    edited_at INTEGER,
    PRIMARY KEY (channel_id, created_at)
);
CREATE TABLE IF NOT EXISTS session (
    user_id INTEGER NOT NULL REFERENCES account(id),
    valid_until INTEGER NOT NULL
);

INSERT INTO account (id,username,email,password_hash,permissions)
     VALUES (0, 'test_admin', 'test_email', 'blahblah', 'admin'),
            (1, 'alice', 'alice_email', 'blahblah', ''),
            (2, 'bob', 'bob_email', 'blahblah', ''),
            (3, 'charlie', 'charlie_email', 'blahblah', '');

INSERT INTO channel (id,name,created_at)
     VALUES (0, 'General', 1730950674183),      -- Jan 1, 2024 11:59:43 AM
            (1, 'Random', 1730950674562),       -- Jan 2, 2024 12:12:42 PM
            (2, 'Announcements', 1730950674147); -- Jan 3, 2024 12:15:47 PM

INSERT INTO message (created_at,creator_id,channel_id,content,edited_at)
     VALUES (1730950674213, 1, 0, '<strong>I am alice!</strong> Hello general channel 👋', NULL),                                              -- Jan 1, 2024 12:13:33 AM
            (1730950674853, 2, 0, 'Hi <em>Alice</em>! ✨ Welcome to <strong>general channel</strong> 🎉', 1730950675213),                        -- Jan 2, 2024 12:04:13 AM (edited)
            (1730950676473, 1, 0, 'Thanks for welcoming me Bob! 🤗', NULL),                                                                    -- Jan 4, 2024 12:07:53 AM
            (1730950677933, 2, 0, 'No problem Alice, its <em>literally</em> my job. They will <strong>actually</strong> fire me. 😅', 1730950678121),     -- Jan 5, 2024 12:02:13 AM (edited)
            (1730950679693, 1, 0, 'Oh, well then I <s>retract my thanks</s>. 😤', NULL),                                                      -- Jan 7, 2024 12:14:53 AM
            (1730950680473, 2, 0, '<em>*sigh*</em>, its a living. 🤷‍♂️', 1730950681242),                                                         -- Jan 8, 2024 12:15:13 AM (edited)
            (1730950682913, 1, 1, 'Can I also post in the <strong>random</strong> channel? 🤔', NULL),                                        -- Jan 8, 2024 02:47:33 AM
            (1730950683473, 2, 1, 'Yeah, we cant stop you. 😏', 1730950685413),          
            (1730950683474, 2, 1, 'Here''s an image <img src="/weird_smile.jpg">', 1730950685413),                                                            -- Jan 8, 2024 02:54:33 AM (edited at 03:04:15)
            (1730950683953, 1, 1, 'LOL! Thats funny 🤣', NULL),                                                                               -- Jan 8, 2024 03:46:33 AM
            (1730950684973, 2, 1, 'No <em>literally</em>, we have <strong>no security</strong> implement yet! 🤫', 1730950686613);              -- Jan 8, 2024 03:53:53 AM (edited at 03:03:42)
