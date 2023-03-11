CREATE TABLE packs (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL
);

CREATE TABLE cards (
    id INTEGER PRIMARY KEY,
    front TEXT NOT NULL,
    back TEXT NOT NULL,
    pack_id INTEGER NOT NULL REFERENCES packs(id)
);
