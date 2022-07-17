DROP TABLE player CASCADE;
DROP TABLE character CASCADE;
DROP TABLE banner CASCADE;
DROP TABLE banner_content CASCADE;
DROP TABLE unique_character CASCADE;
DROP TABLE player_resource CASCADE;


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
    is_origins BOOLEAN DEFAULT false,

    CHECK (rarity >= 0 AND rarity <= 5)
);

CREATE TABLE IF NOT EXISTS banner(
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS banner_content(
    character INTEGER REFERENCES character,
    banner INTEGER REFERENCES banner,

    UNIQUE (character, banner)
);

CREATE TABLE IF NOT EXISTS unique_character(
    id BIGSERIAL PRIMARY KEY,
    character INTEGER REFERENCES character,
    owner BIGINT NOT NULL REFERENCES player,

    UNIQUE (id, owner)
);

CREATE TABLE IF NOT EXISTS player_resource(
    owner BIGINT REFERENCES player,
    summon_ticket_base BIGINT DEFAULT 0 CHECK (summon_ticket_base >= 0)
);
