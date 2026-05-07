CREATE TABLE IF NOT EXISTS signatures (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    max_sections INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL CHECK(type IN ('Client', 'Admin')),
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    phone TEXT,
    active INTEGER NOT NULL DEFAULT 1,
    birthdate TEXT
);

CREATE TABLE IF NOT EXISTS customize_pages (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    title TEXT NOT NULL,
    theme TEXT NOT NULL,
    purchased_in TEXT NOT NULL,
    renewed_in TEXT,
    active INTEGER NOT NULL DEFAULT 1,
    background_music TEXT NOT NULL,
    user_id TEXT NOT NULL, --FK
    signature_id TEXT NOT NULL,  --FK


    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (signature_id) REFERENCES signatures(id)
);

CREATE TABLE IF NOT EXISTS sections (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    "order" INTEGER NOT NULL,
    page_id TEXT NOT NULL, --FK

    FOREIGN KEY (page_id) REFERENCES customize_pages(id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS propertys (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    value TEXT,
    section_id TEXT NOT NULL, --FK

    FOREIGN KEY (section_id) REFERENCES sections(id) ON DELETE CASCADE
);
