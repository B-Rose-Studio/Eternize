-- PAGE
INSERT OR REPLACE INTO customize_pages (id, name, title, theme, background_music, purchased_in, renewed_in, active, user_id, signature_id)
VALUES ('7f1b2c9e-5d4a-438e-9b2f-1e6a8d7c3f4b', 'primeiro-ano-clara', 'Nosso Primeiro Ano', 'MidnightBlue', '', '2026-05-03T12:00:00Z', '2026-05-03T12:00:00Z', 1, 'a1b2c3d4-e5f6-4a5b-bc6d-7e8f9a0b1c2d', '7b1897e0-466d-4b82-9654-2c0692557e84');

-- SECTIONS
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('f2c4e9b1-8d3a-4f5c-b1e7-9a6d3c2b8f4e', 'Hero', 0, '7f1b2c9e-5d4a-438e-9b2f-1e6a8d7c3f4b');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('e5a8d2c9-1f3b-47e6-8c4b-7a9d1f3e2b6c', 'Timeline', 1, '7f1b2c9e-5d4a-438e-9b2f-1e6a8d7c3f4b');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('b3f1e6d4-9c2a-48b5-a7f3-2e4d9c1b8a5f', 'Glass', 2, '7f1b2c9e-5d4a-438e-9b2f-1e6a8d7c3f4b');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b', 'Album', 3, '7f1b2c9e-5d4a-438e-9b2f-1e6a8d7c3f4b');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('6a3c9b2f-8d1e-45a7-b2c4-1e9f3d6a8b7c', 'Gift', 4, '7f1b2c9e-5d4a-438e-9b2f-1e6a8d7c3f4b');

-- PROPERTYS: HERO
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('1d8c4e5a-2f9b-43d6-8e1f-7a3b9c2d5e4f', 'hero_cover_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/sHHbLnjwFtztfrJhqbrDXt.png', 'f2c4e9b1-8d3a-4f5c-b1e7-9a6d3c2b8f4e'),
('8b2f5a1c-9d3e-47c8-a4b6-1e5f7d3c9a2b', 'hero_badge_text', '1º Dia das Mães', 'f2c4e9b1-8d3a-4f5c-b1e7-9a6d3c2b8f4e'),
('4c9e1a3d-6b8f-42a5-9d7c-3f2e8a1b5c4d', 'hero_main_title', 'Você nasceu para isso, Clara', 'f2c4e9b1-8d3a-4f5c-b1e7-9a6d3c2b8f4e'),
('9a3f2b1d-7e5c-48d1-b6e9-4c1a8f3b2d5e', 'hero_subtitle_message', 'O primeiro ano da nossa maior aventura. Ver você se tornar mãe é o maior presente da minha vida.', 'f2c4e9b1-8d3a-4f5c-b1e7-9a6d3c2b8f4e'),
('3f5d1e2a-9b4c-46a8-8c7b-2e1d9f3a5b6c', 'hero_highlight_items', '["Cuidado","Noites em claro","Amor puro"]', 'f2c4e9b1-8d3a-4f5c-b1e7-9a6d3c2b8f4e');

-- PROPERTYS: TIMELINE
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('5e2c8a1b-3f9d-41b7-9a4e-6d7c2f5b8a3e', 'timeline_section_title', 'Nossos Primeiros Passos', 'e5a8d2c9-1f3b-47e6-8c4b-7a9d1f3e2b6c'),
('7d1f3e2b-8a4c-49d5-b2c6-9e5a1b3f4d7c', 'timeline_events', '[ {"year": "2025","description": "O dia do positivo. Nosso mundo virou de cabeça para baixo!","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_0.png"},{"year": "2025","description": "A barriga crescendo e a ansiedade tomando conta da casa.","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_1.png"},{"year": "2026","description": "Nosso pacotinho nos braços. A família está completa.","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_2.png"}]', 'e5a8d2c9-1f3b-47e6-8c4b-7a9d1f3e2b6c');

-- PROPERTYS: GLASS
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('2a9b4c1d-5e3f-48b2-a7d6-1c8f3e5b9a4c', 'glass_background_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/qZxZ4F1Pdbg44kDezcDiVG.png', 'b3f1e6d4-9c2a-48b5-a7f3-2e4d9c1b8a5f'),
('8c3e1a5f-4d9b-47c1-9b2e-6f8a3d2c7b1e', 'glass_card_one_title', 'Uma Nova Melodia', 'b3f1e6d4-9c2a-48b5-a7f3-2e4d9c1b8a5f'),
('f5b2d1c9-a8e3-46f4-8d1a-3c7b9e4a2f5d', 'glass_card_one_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/1ki7EM3rmAceSExsDgjCvo.png', 'b3f1e6d4-9c2a-48b5-a7f3-2e4d9c1b8a5f'),
('1e4a8f3b-2c7d-45e9-b6a1-9d5c2b8e3f4a', 'glass_card_two_text', 'As madrugadas são longas, mas eu filmei cada sorriso roubado para te lembrar do quanto somos felizes.', 'b3f1e6d4-9c2a-48b5-a7f3-2e4d9c1b8a5f'),
('d6c2a9b1-5e8f-43d7-a1b4-7f3e2a9c5b8d', 'glass_card_three_cta_text', 'Ver Retrospectiva', 'b3f1e6d4-9c2a-48b5-a7f3-2e4d9c1b8a5f'),
('3b8e5f1a-7c2d-49a6-8e3f-4a1b9d6c2e5f', 'glass_card_three_video_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/nDRv3ueREpcubhNRTxZAUp.mp4', 'b3f1e6d4-9c2a-48b5-a7f3-2e4d9c1b8a5f');

-- PROPERTYS: ALBUM
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('9f4d1c2b-6a8e-42b5-b7c3-5e1a8f9d3b4c', 'album_section_title', 'Retratos de Amor', 'c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b'),
('4a1b6c3d-8e2f-45a9-9d7b-2f5e1a3c8b9d', 'album_photo_one_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/2MJiPyLXyhjSzFkaAaqVH1.png', 'c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b'),
('c7e2d5f1-3a9b-41c8-8b4e-6a9d2c3f5b1e', 'album_photo_one_caption', 'O sono mais gostoso e seguro do mundo.', 'c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b'),
('5b9c2e1a-4f8d-47b3-a6d5-1c8f3e2b9a4d', 'album_photo_two_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oALzuPmCpGPFUtzdEKKnWU.png', 'c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b'),
('1a8f3e5b-7d2c-49a1-b4c6-9d5e2a1b8c3f', 'album_photo_two_badge_text', 'Mãe Coruja', 'c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b'),
('f3b2d1e9-5a8c-46f7-8d1b-3c4a9f2e5b7d', 'album_photo_two_caption', 'O seu olhar apaixonado não esconde nada.', 'c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b'),
('8e1a5c3f-2d9b-48e4-a7c2-6f9b3d1a4e5c', 'album_photo_three_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/teVzYM5T61wLRfeD9fTanD.png', 'c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b'),
('2d7c4a1b-9e5f-43d8-b1a6-5e8f3c2b9d4a', 'album_photo_three_caption', 'Nosso domingo de pernas pro ar.', 'c9d2a1f5-3e8b-41c7-9b6e-4f8a2c3d7e1b');

-- PROPERTYS: GIFT
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('7f3a9b2d-1e5c-46a8-9d4b-8c1a5f3e2b6d', 'gift_teaser_text', 'Você merece um descanso...', '6a3c9b2f-8d1e-45a7-b2c4-1e9f3d6a8b7c'),
('b5e1d2c9-8f3a-44b7-a2e6-7a9c1b3f4d5e', 'gift_interaction_instruction', 'Revele sua noite', '6a3c9b2f-8d1e-45a7-b2c4-1e9f3d6a8b7c'),
('3c8a1b5f-9d2e-47c1-8b5e-4f1a9d3c2b8e', 'gift_reveal_title', 'Um Vale-Night!', '6a3c9b2f-8d1e-45a7-b2c4-1e9f3d6a8b7c'),
('6d2e9f4c-1a8b-45d3-b7f2-5e3a8c1b9d4f', 'gift_surprise_name', 'Jantar Romântico a Dois', '6a3c9b2f-8d1e-45a7-b2c4-1e9f3d6a8b7c'),
('e9a4b1c3-5d7f-48e2-a1c6-2b8f3d5a9e1c', 'gift_surprise_message', 'A avó já confirmou presença para ficar com o bebê amanhã à noite. Arrume-se sem pressa, a mesa no seu restaurante favorito já está reservada.', '6a3c9b2f-8d1e-45a7-b2c4-1e9f3d6a8b7c'),
('1b5f3e2a-8c9d-41a6-9e4b-7d2c1a8f5b3e', 'gift_surprise_action_url', '', '6a3c9b2f-8d1e-45a7-b2c4-1e9f3d6a8b7c'),
('4c1a8f3b-5e2d-49b7-8a1c-6d9e2b5f3a4c', 'gift_surprise_button_label', 'Ver Reserva', '6a3c9b2f-8d1e-45a7-b2c4-1e9f3d6a8b7c');
