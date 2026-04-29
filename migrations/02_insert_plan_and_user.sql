INSERT OR IGNORE INTO signatures (id, name, price, max_sections)
VALUES ('7b1897e0-466d-4b82-9654-2c0692557e84', 'Basic', 25.00, 4);

-- (password: admin123) Bcrypt with 12 rounds
INSERT OR IGNORE INTO users (id, type, email, password, first_name, last_name, active)
VALUES (
    'a1b2c3d4-e5f6-4a5b-bc6d-7e8f9a0b1c2d',
    'Admin',
    'admin@bluerosestudio.com',
    '$2a$12$N9qx1y03Z41D7Q7g1bV5U.rD/61P.k82Q/cK./gS1q81y32Z.496K',
    'Admin',
    'Blue Rose',
    1
);
