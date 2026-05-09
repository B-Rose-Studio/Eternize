-- PAGE
INSERT OR REPLACE INTO customize_pages (id, name, title, theme, background_music, purchased_in, renewed_in, active, user_id, signature_id)
VALUES ('73e6a921-2f1d-4c8a-9b4e-8f2a1c0d5e9b', 'minha-inspiracao-sonia', 'Nossa Inspiração', 'Obsidian', '', '2026-05-03T12:00:00Z', '2026-05-03T12:00:00Z', 1, 'a1b2c3d4-e5f6-4a5b-bc6d-7e8f9a0b1c2d', '7b1897e0-466d-4b82-9654-2c0692557e84');

-- SECTIONS
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d', 'Hero', 0, '73e6a921-2f1d-4c8a-9b4e-8f2a1c0d5e9b');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('f9e8d7c6-b5a4-4321-8b7c-6d5e4f3a2b1c', 'Glass', 1, '73e6a921-2f1d-4c8a-9b4e-8f2a1c0d5e9b');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f', 'Album', 2, '73e6a921-2f1d-4c8a-9b4e-8f2a1c0d5e9b');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('2b1a4c3d-5e6f-4a7b-8c9d-0e1f2a3b4c5d', 'Timeline', 3, '73e6a921-2f1d-4c8a-9b4e-8f2a1c0d5e9b');
INSERT OR REPLACE INTO sections (id, name, 'order', page_id) VALUES('7d6e5f4a-3b2c-4d1e-8f9a-0b1c2d3e4f5a', 'Gift', 4, '73e6a921-2f1d-4c8a-9b4e-8f2a1c0d5e9b');

-- PROPERTYS: HERO
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('5c9e1a2b-3d4f-4a5b-8c9d-0e1f2a3b4c5d', 'hero_cover_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/sHHbLnjwFtztfrJhqbrDXt.png', 'a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d'),
('8d7c6b5a-4321-4f9e-8d7c-6b5a43210f9e', 'hero_badge_text', 'Elegância & Força', 'a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d'),
('f1e2d3c4-b5a6-407b-8c9d-0e1f2a3b4c5d', 'hero_main_title', 'O exemplo de garra que me guia, Sônia', 'a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d'),
('a9b8c7d6-e5f4-4321-8b7c-6d5e4f3a2b1c', 'hero_subtitle_message', 'Mãe solo, executiva e o nosso pilar mais sólido. Você fez o impossível parecer fácil todos os dias.', 'a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d'),
('b5a43210-f9e8-4d7c-6b5a-43210f9e8d7c', 'hero_highlight_items', '["Independência","Garra","Sofisticação"]', 'a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d');

-- PROPERTYS: GLASS
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('4c3d2b1a-0e9f-4a7b-8c9d-0e1f2a3b4c5d', 'glass_background_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/qZxZ4F1Pdbg44kDezcDiVG.png', 'f9e8d7c6-b5a4-4321-8b7c-6d5e4f3a2b1c'),
('1f2a3b4c-5d6e-4f7a-8b9c-0d1e2f3a4b5c', 'glass_card_one_title', 'Legado Inestimável', 'f9e8d7c6-b5a4-4321-8b7c-6d5e4f3a2b1c'),
('9e8d7c6b-5a43-4210-9b8c-7d6e5f4a3210', 'glass_card_one_image_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/1ki7EM3rmAceSExsDgjCvo.png', 'f9e8d7c6-b5a4-4321-8b7c-6d5e4f3a2b1c'),
('a1b2c3d4-e5f6-4a5b-bc6d-7e8f9a0b1c2d', 'glass_card_two_text', 'Tudo o que eu sou hoje, minha educação e meus valores, eu devo inteiramente a você.', 'f9e8d7c6-b5a4-4321-8b7c-6d5e4f3a2b1c'),
('d4c3b2a1-0e9f-4876-b543-210987654321', 'glass_card_three_cta_text', 'Carta Aberta', 'f9e8d7c6-b5a4-4321-8b7c-6d5e4f3a2b1c'),
('5e6f7a8b-9c0d-41e2-b3c4-d5e6f7a8b9c0', 'glass_card_three_video_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/nDRv3ueREpcubhNRTxZAUp.mp4', 'f9e8d7c6-b5a4-4321-8b7c-6d5e4f3a2b1c');

-- PROPERTYS: ALBUM
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('b1c2d3e4-f5a6-47b8-9c0d-1e2f3a4b5c6d', 'album_section_title', 'Momentos Brilhantes', 'c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f'),
('c2d3e4f5-a6b7-48c9-d0e1-2f3a4b5c6d7e', 'album_photo_one_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/2MJiPyLXyhjSzFkaAaqVH1.png', 'c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f'),
('d3e4f5a6-b7c8-49d9-e1f2-3a4b5c6d7e8f', 'album_photo_one_caption', 'Sua promoção que comemoramos tanto.', 'c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f'),
('e4f5a6b7-c8d9-40ea-f23a-4b5c6d7e8f9a', 'album_photo_two_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oALzuPmCpGPFUtzdEKKnWU.png', 'c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f'),
('f5a6b7c8-d9e0-41fb-034b-5c6d7e8f9a0b', 'album_photo_two_badge_text', 'Parceria', 'c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f'),
('a6b7c8d9-0e1f-42bc-145c-6d7e8f9a0b1c', 'album_photo_two_caption', 'Aquela nossa viagem que mudou tudo.', 'c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f'),
('b7c8d9e0-1f2a-43cd-256d-7e8f9a0b1c2d', 'album_photo_three_url', 'https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/teVzYM5T61wLRfeD9fTanD.png', 'c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f'),
('c8d9e0f1-2a3b-44de-367e-8f9a0b1c2d3e', 'album_photo_three_caption', 'Brindando à vida e a nós duas.', 'c1d2e3f4-a5b6-4c7d-8e9f-0a1b2c3d4e5f');

-- PROPERTYS: TIMELINE
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('d9e0f1a2-3b4c-45ef-478f-9a0b1c2d3e4f', 'timeline_section_title', 'Escalada para o Sucesso', '2b1a4c3d-5e6f-4a7b-8c9d-0e1f2a3b4c5d'),
('e0f1a2b3-4c5d-46f0-589a-0b1c2d3e4f5a', 'timeline_events', '[ {"year": "2005","description": "Quando você assumiu as rédeas e começou a jornada do zero.","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_0.png"},{"year": "2015","description": "Sua formatura e a minha, no mesmo ano. Um orgulho duplo!","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_1.png"},{"year": "2026","description": "Hoje, a mulher de negócios que eu me espelho todos os dias.","image_url": "https://eternize-storage.bluerosestudio.com.br/pages/jyQzcvGKcDv6fC5Wp2qvHE/oQtuv88FHk6inMecnCYFL5_2.png"}]', '2b1a4c3d-5e6f-4a7b-8c9d-0e1f2a3b4c5d');

-- PROPERTYS: GIFT
INSERT OR REPLACE INTO propertys (id, name, value, section_id) VALUES
('f1a2b3c4-5d6e-47a0-69b1-c2d3e4f5a6b7', 'gift_teaser_text', 'É hora de bater o ponto...', '7d6e5f4a-3b2c-4d1e-8f9a-0b1c2d3e4f5a'),
('a2b3c4d5-6e7f-48b1-7ac2-d3e4f5a6b7c8', 'gift_interaction_instruction', 'Revelar Destino', '7d6e5f4a-3b2c-4d1e-8f9a-0b1c2d3e4f5a'),
('b3c4d5e6-7f8a-49c2-8bd3-e4f5a6b7c8d9', 'gift_reveal_title', 'Férias Merecidas!', '7d6e5f4a-3b2c-4d1e-8f9a-0b1c2d3e4f5a'),
('c4d5e6f7-a8b9-40d3-9ce4-f5a6b7c8d9e0', 'gift_surprise_name', 'Passagens para Europa', '7d6e5f4a-3b2c-4d1e-8f9a-0b1c2d3e4f5a'),
('d5e6f7a8-b9c0-41e4-0df5-a6b7c8d9e0f1', 'gift_surprise_message', 'Mãe, desliga o notebook! O seu voo para a Itália sai semana que vem e eu vou com você para carregarmos as malas das nossas compras.', '7d6e5f4a-3b2c-4d1e-8f9a-0b1c2d3e4f5a'),
('e6f7a8b9-c0d1-42f5-1e06-b7c8d9e0f1a2', 'gift_surprise_action_url', '', '7d6e5f4a-3b2c-4d1e-8f9a-0b1c2d3e4f5a'),
('f7a8b9c0-d1e2-4306-2f17-c8d9e0f1a2b3', 'gift_surprise_button_label', 'Baixar E-Tickets', '7d6e5f4a-3b2c-4d1e-8f9a-0b1c2d3e4f5a');
