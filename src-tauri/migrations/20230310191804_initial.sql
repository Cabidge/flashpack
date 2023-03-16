CREATE TABLE packs (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE cards (
    id INTEGER PRIMARY KEY,
    front TEXT NOT NULL,
    back TEXT NOT NULL,
    pack_id INTEGER NOT NULL REFERENCES packs(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL
);

CREATE TABLE card_tags (
    card_id INTEGER NOT NULL REFERENCES cards(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    PRIMARY KEY (card_id, tag_id)
);
