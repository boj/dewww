-- Add up migration script here
CREATE TABLE domains (
    id INTEGER PRIMARY KEY,
    base_url TEXT NOT NULL UNIQUE,
    added DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_domains ON domains (base_url);

CREATE TABLE domain_local_links (
    id INTEGER PRIMARY KEY,
    domain_id INTEGER NOT NULL,
    link_url TEXT NOT NULL UNIQUE,
    title TEXT,
    body TEXT,
    added DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX idx_domain_local_links ON domain_local_links (domain_id, link_url);

CREATE VIRTUAL TABLE domain_content_fts USING fts5(
    title,
    body,
    content='domain_local_links',
    content_rowid='id'
);
CREATE TRIGGER domain_content_ai AFTER INSERT ON domain_local_links
    BEGIN
        INSERT INTO domain_content_fts (rowid, title, body)
        VALUES (new.id, new.title, new.body);
    END;
CREATE TRIGGER domain_content_ad AFTER DELETE ON domain_local_links
    BEGIN
        INSERT INTO domain_content_fts (domain_content_fts, rowid, title, body)
        VALUES ('delete', old.id, old.title, old.body);
    END;
CREATE TRIGGER domain_content_au AFTER UPDATE ON domain_local_links
    BEGIN
        INSERT INTO domain_content_fts (domain_content_fts, rowid, title, body)
        VALUES ('delete', old.id, old.title, old.body);
        INSERT INTO domain_content_fts (rowid, title, body)
        VALUES (new.id, new.title, new.body);
    END;