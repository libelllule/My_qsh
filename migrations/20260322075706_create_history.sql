CREATE TABLE IF NOT EXISTS history (
    note_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES "user"(uid),
    hfilename TEXT NOT NULL,
    hsize TEXT NOT NULL,
    hdate DATETIME NOT NULL,
    hstatus CHAR(1) NOT NULL CHECK (hstatus IN ('u','d'))
);
