CREATE TABLE study_queries (
    id INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    pack_id INTEGER REFERENCES packs(id)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    question_count INTEGER NOT NULL
);

CREATE TABLE study_query_tags (
    study_id INTEGER NOT NULL REFERENCES study_queries(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    tag TEXT NOT NULL COLLATE NOCASE,
    exclude BOOLEAN DEFAULT FALSE NOT NULL,
    PRIMARY KEY (study_id, tag, exclude)
);
