CREATE TABLE IF NOT EXISTS history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES "user"(uid),
    filename TEXT NOT NULL,
    size TEXT NOT NULL,
    date DATETIME NOT NULL,
    -- Statuses are: 'u' - upload, 'd' - download
    status CHAR(1) NOT NULL CHECK (status IN ('u','d'))
);
