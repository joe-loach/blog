DROP TABLE IF EXISTS posts;

CREATE TABLE
    IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        -- expected to be {
        --     title: "...",
        --     date: "...",
        --     key: "...",
        --     tags: ["...", ..]
        -- }
        meta TEXT,
        title AS (json_extract(meta, '$.title')) STORED,
        date AS (json_extract(meta, '$.date')) STORED
    );