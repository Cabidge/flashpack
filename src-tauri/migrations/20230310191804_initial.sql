CREATE TABLE packs (
    id INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE cards (
    id INTEGER NOT NULL PRIMARY KEY,
    front TEXT NOT NULL,
    back TEXT NOT NULL,
    pack_id INTEGER NOT NULL REFERENCES packs(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE card_tags (
    card_id INTEGER NOT NULL REFERENCES cards(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    tag TEXT NOT NULL COLLATE NOCASE,
    PRIMARY KEY (card_id, tag)
) WITHOUT ROWID;

CREATE INDEX tag_index ON card_tags (tag);

CREATE TABLE dealers (
    id INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE filters (
    id INTEGER NOT NULL PRIMARY KEY,
    label TEXT NOT NULL,
    pack_id INTEGER NOT NULL REFERENCES packs(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

CREATE TABLE included_tags (
    filter_id INTEGER NOT NULL REFERENCES filters(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    tag TEXT NOT NULL COLLATE NOCASE,
    PRIMARY KEY (filter_id, tag)
) WITHOUT ROWID;


CREATE TABLE excluded_tags (
    filter_id INTEGER NOT NULL REFERENCES filters(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    tag TEXT NOT NULL COLLATE NOCASE,
    PRIMARY KEY (filter_id, tag)
) WITHOUT ROWID;

CREATE TABLE dealer_filters (
    dealer_id INTEGER NOT NULL REFERENCES dealers(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    filter_id INTEGER REFERENCES filters(id)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    strength INTEGER NOT NULL,
    PRIMARY KEY (dealer_id, filter_id)
);
