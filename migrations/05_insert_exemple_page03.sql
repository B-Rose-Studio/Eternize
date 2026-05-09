-- PAGE
INSERT OR REPLACE INTO customize_pages (id, name, title, theme, background_music, purchased_in, renewed_in, active, user_id, signature_id)
VALUES ('573e8a1d-4b92-4f2c-8a19-3d6f7e2c9b4a', 'amor-em-dobro-maria', 'Amor em Dobro', 'Emerald', '', '2026-05-03T12:00:00Z', '2026-05-03T12:00:00Z', 1, 'a1b2c3d4-e5f6-4a5b-bc6d-7e8f9a0b1c2d', '7b1897e0-466d-4b82-9654-2c0692557e84');

-- SECTIONS
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('c4b1d9a2-7f3e-48c5-9e1d-2a6f8b3c4e7d', 'Hero', 0, '573e8a1d-4b92-4f2c-8a19-3d6f7e2c9b4a');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a', 'Album', 1, '573e8a1d-4b92-4f2c-8a19-3d6f7e2c9b4a');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('9e2d4a1f-8c3b-45d7-a6f9-1e8b2c4a3d5f', 'Timeline', 2, '573e8a1d-4b92-4f2c-8a19-3d6f7e2c9b4a');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('1f3a5e8c-2d9b-46c1-b7a4-8e9f2d3c1b4a', 'Glass', 3, '573e8a1d-4b92-4f2c-8a19-3d6f7e2c9b4a');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('6b4c9e1a-5f2d-48a3-9d7e-3c1b2f4a8e5d', 'Gift', 4, '573e8a1d-4b92-4f2c-8a19-3d6f7e2c9b4a');

-- PROPERTYS: HERO
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('4d1a9e3f-6b8c-42d5-a1f7-2c5e8b9a3d4f', 'hero_cover_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/sHHbLnjwFtztfrJhqbrDXt.png', 'c4b1d9a2-7f3e-48c5-9e1d-2a6f8b3c4e7d'),
('7e2f5b8a-1c9d-43e6-b4a8-9d3c1f2e5a6b', 'hero_badge_text', 'Mãe & Avó', 'c4b1d9a2-7f3e-48c5-9e1d-2a6f8b3c4e7d'),
('2a9c1d4e-8f3b-47a2-9e5c-6b1a3d8f4e2c', 'hero_main_title', 'A raiz da nossa árvore, Dona Maria', 'c4b1d9a2-7f3e-48c5-9e1d-2a6f8b3c4e7d'),
('f3e8a2b5-9d1c-46f8-8b2a-4c7e1d5a9b3f', 'hero_subtitle_message', 'Mãe é um instinto que nunca apaga, mas ser avó é amar pela segunda vez com ainda mais doçura.', 'c4b1d9a2-7f3e-48c5-9e1d-2a6f8b3c4e7d'),
('5b7c3d1f-4a8e-49b2-a6d9-1e2f8c5a3b4d', 'hero_highlight_items', '["Legado","Sabedoria","Colo de Vó"]', 'c4b1d9a2-7f3e-48c5-9e1d-2a6f8b3c4e7d');

-- PROPERTYS: ALBUM
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('d8a2f1c9-3b4e-41d7-b5e8-6a9c2f3b4e1d', 'album_section_title', 'Colheita de Amor', '3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a'),
('1c5e9a2b-8f3d-46c4-9a7b-3d4e1f8a2c5b', 'album_photo_one_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/2MJiPyLXyhjSzFkaAaqVH1.png', '3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a'),
('e9f2b8a1-4c7d-45e3-a2b6-8c1d5f3e9a4b', 'album_photo_one_caption', 'Você e as orquídeas que cuidamos juntas.', '3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a'),
('8a3d1c5e-9b2f-48a6-b4c9-2e7f1a3b8d4c', 'album_photo_two_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oALzuPmCpGPFUtzdEKKnWU.png', '3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a'),
('b6c4e2a9-1f8d-43b5-9e1d-7a5b3c4f8e2a', 'album_photo_two_badge_text', 'A Matriarca', '3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a'),
('4e1f8a3d-2b9c-47d1-a5c8-6b2e9d1a4f3b', 'album_photo_two_caption', 'Sempre cercada por todos nós na mesa de domingo.', '3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a'),
('9d2c5b1a-8f4e-46a2-b3e7-1c4a8f5d2b9e', 'album_photo_three_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/teVzYM5T61wLRfeD9fTanD.png', '3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a'),
('c1a4d9e2-5b3f-48c7-9f2a-8b1e4c3d7a6f', 'album_photo_three_caption', 'O chá da tarde com o seu neto favorito.', '3a8f2c9e-1d4b-47e6-8c5a-9b2f1e3d4c6a');

-- PROPERTYS: TIMELINE
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('3f8e1a4c-9d2b-45f6-a7c3-2b6d8e1a5f4c', 'timeline_section_title', 'Sementes Plantadas', '9e2d4a1f-8c3b-45d7-a6f9-1e8b2c4a3d5f'),
('6a2b9c3d-1e8f-44a1-b5d8-9c4f2a1e3b7d', 'timeline_events', '[ {"year": "1990","description": "A mudança para a casa nova. Lembra do quintal sem nada?","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_0.png"},{"year": "2010","description": "O dia em que você foi promovida a avó. Uma emoção sem igual.","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_1.png"},{"year": "2026","description": "Hoje, com a família reunida e o quintal cheio das suas flores.","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_2.png"}]', '9e2d4a1f-8c3b-45d7-a6f9-1e8b2c4a3d5f');

-- PROPERTYS: GLASS
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('7c5d2a8b-4f1e-49b3-a6e1-3d9f4c2b8a5e', 'glass_background_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/qZxZ4F1Pdbg44kDezcDiVG.png', '1f3a5e8c-2d9b-46c1-b7a4-8e9f2d3c1b4a'),
('2b9e4a1f-8c5d-43e7-b1a9-6f3d2c5e8b4a', 'glass_card_one_title', 'Raízes Profundas', '1f3a5e8c-2d9b-46c1-b7a4-8e9f2d3c1b4a'),
('f5a1d8c2-3e9b-46f4-8c2e-1a7b4d9e5f3c', 'glass_card_one_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/1ki7EM3rmAceSExsDgjCvo.png', '1f3a5e8c-2d9b-46c1-b7a4-8e9f2d3c1b4a'),
('a8d3c1e5-2f4a-49b6-9e7d-5b8c1a2f4e9d', 'glass_card_two_text', 'A família inteira parou um minutinho do dia para gravar algo especial para você.', '1f3a5e8c-2d9b-46c1-b7a4-8e9f2d3c1b4a'),
('1e6f9a2b-5d8c-47a1-a4b3-8c2e1d5f9a4b', 'glass_card_three_cta_text', 'Assistir Recados', '1f3a5e8c-2d9b-46c1-b7a4-8e9f2d3c1b4a'),
('4b2a8c5d-9e1f-43d8-b6a2-7f1d3e4c9b5a', 'glass_card_three_video_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/nDRv3ueREpcubhNRTxZAUp.mp4', '1f3a5e8c-2d9b-46c1-b7a4-8e9f2d3c1b4a');

-- PROPERTYS: GIFT
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('5c9d1a3b-2e8f-45c6-a1b7-4e6f2a8c9d1b', 'gift_teaser_text', 'Tem um pacotinho te esperando...', '6b4c9e1a-5f2d-48a3-9d7e-3c1b2f4a8e5d'),
('e3f8b2a1-4d7c-41e9-9c5d-8a1b6f3e2d4c', 'gift_interaction_instruction', 'Revelar Surpresa', '6b4c9e1a-5f2d-48a3-9d7e-3c1b2f4a8e5d'),
('8a1b4c9d-5e2f-46a3-b8d1-2c7f5a3e1b9d', 'gift_reveal_title', 'Um Jardim Novo!', '6b4c9e1a-5f2d-48a3-9d7e-3c1b2f4a8e5d'),
('2d5e9f1a-8c3b-48d7-a4e2-6b1a9c3d5f8e', 'gift_surprise_name', 'Móveis Novos para Varanda', '6b4c9e1a-5f2d-48a3-9d7e-3c1b2f4a8e5d'),
('c7b2a5d8-1f9e-43c4-9e1f-3d8a2c5b4e6a', 'gift_surprise_message', 'Mãe, juntamos todo mundo e compramos aquele conjunto de cadeiras e mesa de vime que você vivia olhando na loja. Chega amanhã!', '6b4c9e1a-5f2d-48a3-9d7e-3c1b2f4a8e5d'),
('9f3c1e4a-6b8d-47a2-b5c9-1a4d8e2f5b3c', 'gift_surprise_action_url', '', '6b4c9e1a-5f2d-48a3-9d7e-3c1b2f4a8e5d'),
('3a6b8d1c-4e9f-42b5-a8d3-5c2f9a1e7b4d', 'gift_surprise_button_label', 'Ver Foto do Conjunto', '6b4c9e1a-5f2d-48a3-9d7e-3c1b2f4a8e5d');
