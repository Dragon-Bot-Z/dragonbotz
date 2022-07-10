CREATE TABLE IF NOT EXISTS player(
    discord_id BIGINT PRIMARY KEY,
    register_date DATE DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS character(
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    rarity SMALLINT DEFAULT 0,
    image TEXT DEFAULT 'https://i.imgur.com/eMiHxeP.png',
    thumbnail TEXT DEFAULT 'https://i.imgur.com/eMiHxeP.png'
);