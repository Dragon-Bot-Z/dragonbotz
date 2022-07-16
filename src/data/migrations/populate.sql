-- inserting test values for player table
INSERT INTO player VALUES(136160890000113664, '2022-07-09');

-- inserting test values for character table
INSERT INTO character(name, rarity, image, thumbnail) 
    VALUES('Thanus', 5, 'https://i.imgur.com/hwsOe06.png', 'https://i.imgur.com/QC3aaUU.png');

INSERT INTO character(name, rarity, image, thumbnail) 
    VALUES('Thanus_ultra', 4, 'https://i.imgur.com/hwsOe06.png', 'https://i.imgur.com/QC3aaUU.png');

INSERT INTO character(name, rarity, image, thumbnail) 
    VALUES('Thanus_extreme', 3, 'https://i.imgur.com/hwsOe06.png', 'https://i.imgur.com/QC3aaUU.png');

INSERT INTO character(name, rarity, image, thumbnail) 
    VALUES('Thanus_super', 2, 'https://i.imgur.com/hwsOe06.png', 'https://i.imgur.com/QC3aaUU.png');    

INSERT INTO character(name, rarity, image, thumbnail) 
    VALUES('Thanus_uncommon', 1, 'https://i.imgur.com/hwsOe06.png', 'https://i.imgur.com/QC3aaUU.png');

INSERT INTO character(name, rarity, image, thumbnail) 
    VALUES('Thanus_common', 0, 'https://i.imgur.com/hwsOe06.png', 'https://i.imgur.com/QC3aaUU.png');

INSERT INTO character(name, rarity, image, thumbnail, is_origins) 
    VALUES('Thanus_origins', 3, 'https://i.imgur.com/hwsOe06.png', 'https://i.imgur.com/QC3aaUU.png', true);


-- inserting test values for banner table
INSERT INTO banner VALUES(DEFAULT, 'Test Banner');

-- inserting test values for banner content
-- test banner
INSERT INTO banner_content VALUES(1, 1);
INSERT INTO banner_content VALUES(2, 1);
INSERT INTO banner_content VALUES(3, 1);
INSERT INTO banner_content VALUES(3, 1);
INSERT INTO banner_content VALUES(4, 1);
INSERT INTO banner_content VALUES(5, 1);
INSERT INTO banner_content VALUES(6, 1);
INSERT INTO banner_content VALUES(7, 1);
