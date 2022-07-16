DROP TABLE player CASCADE;
DROP TABLE character CASCADE;
DROP TABLE banner CASCADE;
DROP TABLE banner_content CASCADE;


CREATE TABLE IF NOT EXISTS player(
    discord_id BIGINT PRIMARY KEY,
    register_date DATE DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS character(
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    rarity SMALLINT DEFAULT 0,
    image TEXT DEFAULT 'https://i.imgur.com/eMiHxeP.png',
    thumbnail TEXT DEFAULT 'https://i.imgur.com/eMiHxeP.png',
    is_origin BOOLEAN DEFAULT false,

    CHECK (rarity >= 0 AND rarity <= 5)
);

CREATE TABLE IF NOT EXISTS banner(
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS banner_content(
    character INTEGER REFERENCES character,
    banner INTEGER REFERENCES banner,

    UNIQUE (character, banner),
    CHECK (character.rarity <= 3)
);
