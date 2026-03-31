CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mac_address TEXT NOT NULL,
    device_name TEXT NOT NULL,
    nickname TEXT,
    status CHAR(1) NOT NULL CHECK (status IN ('b','u','w'))
);
