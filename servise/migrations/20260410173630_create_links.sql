
CREATE TABLE links (
    id         SERIAL PRIMARY KEY,
    code       VARCHAR(10) UNIQUE NOT NULL,
    url        TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_links_code ON links(code);