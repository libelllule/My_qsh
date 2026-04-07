CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mac_address TEXT NOT NULL,
    device_name TEXT NOT NULL,
    nickname TEXT,
    -- Statuses are: 'b' - blocked, 'u' - unknown, 't' - trusted
    status CHAR(1) NOT NULL CHECK (status IN ('b','u','t'))
);
