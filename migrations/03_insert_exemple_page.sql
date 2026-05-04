-- PAGE
INSERT OR IGNORE INTO customize_pages (id, name, title, purchased_in, renewed_in, active, user_id, signature_id)
VALUES (
    '965a3af6-128b-4a7d-883b-d139843c3034',
    'example-page',
    'Example',
    '2026-05-03T12:00:00Z',
    '2026-05-03T12:00:00Z',
    1,
    'a1b2c3d4-e5f6-4a5b-bc6d-7e8f9a0b1c2d',
    '7b1897e0-466d-4b82-9654-2c0692557e84'
);

-- SECTIONS
INSERT OR IGNORE INTO sections (id, name, 'order', page_id)
VALUES('c8e6e57b-7160-4790-806c-49b945cec382','Hero', 0, '965a3af6-128b-4a7d-883b-d139843c3034');

INSERT OR IGNORE INTO sections (id, name, 'order', page_id)
VALUES('cf96b900-3968-4d38-95df-dfe26ecd6ecb','Album', 1, '965a3af6-128b-4a7d-883b-d139843c3034');

INSERT OR IGNORE INTO sections (id, name, 'order', page_id)
VALUES('25d135a0-77f1-49d8-bec9-ed93261ab9f5','Glass', 2, '965a3af6-128b-4a7d-883b-d139843c3034');

INSERT OR IGNORE INTO sections (id, name, 'order', page_id)
VALUES('0c803974-8086-401c-817f-6ab18a190c55','Timeline', 3, '965a3af6-128b-4a7d-883b-d139843c3034');

INSERT OR IGNORE INTO sections (id, name, 'order', page_id)
VALUES('8af963f5-add4-40da-892e-1a2d9714548b','Gift', 4, '965a3af6-128b-4a7d-883b-d139843c3034');

-- PROPERTYS: HERO
INSERT OR IGNORE INTO propertys (id, name, value, section_id)
    VALUES
    ('d8607b33-cdd0-4512-9fad-273e7c50f04d','hero_cover_image_url', '', 'c8e6e57b-7160-4790-806c-49b945cec382'),
    ('4032ee78-2244-4fdd-9a4f-7b8671d65d9c','hero_badge_text', '', 'c8e6e57b-7160-4790-806c-49b945cec382'),
    ('a3ea9340-7c6d-421c-8b03-8d6eed315610','hero_main_title', '', 'c8e6e57b-7160-4790-806c-49b945cec382'),
    ('c20016c5-7b4c-4b22-b80c-a6bacc6c15d8','hero_subtitle_message', '', 'c8e6e57b-7160-4790-806c-49b945cec382'),
    ('5f2c435b-63c6-4561-b737-e5d6caf62944','hero_highlight_items', '[]', 'c8e6e57b-7160-4790-806c-49b945cec382');

-- PROPERTYS: ALBUM
INSERT OR IGNORE INTO propertys (id, name, value, section_id)
    VALUES
    ('f4d21e65-9a81-452f-bc33-9246f4354b4a','album_section_title', '', 'cf96b900-3968-4d38-95df-dfe26ecd6ecb'),
    ('0e7bae5a-eaf5-4cf7-9f44-8709c89b430e','album_photo_one_url', '', 'cf96b900-3968-4d38-95df-dfe26ecd6ecb'),
    ('b9cd3dae-a582-4c7f-8748-e4f1a152f720','album_photo_one_caption', '', 'cf96b900-3968-4d38-95df-dfe26ecd6ecb'),
    ('b703cf82-273b-4b80-b5c7-78203693e90c','album_photo_two_url', '', 'cf96b900-3968-4d38-95df-dfe26ecd6ecb'),
    ('55aecebb-8341-4103-b4e3-59f316933c31','album_photo_two_badge_text', '', 'cf96b900-3968-4d38-95df-dfe26ecd6ecb'),
    ('f2300e15-766d-4920-a251-81569a9ff370','album_photo_two_caption', '', 'cf96b900-3968-4d38-95df-dfe26ecd6ecb'),
    ('dc986faf-fc28-47ad-b231-76efa5c48cc3','album_photo_three_url', '', 'cf96b900-3968-4d38-95df-dfe26ecd6ecb'),
    ('3cf583e7-7f02-48ef-b714-88669b12a534','album_photo_three_caption', '', 'cf96b900-3968-4d38-95df-dfe26ecd6ecb');

-- PROPERTYS: GLASS
INSERT OR IGNORE INTO propertys (id, name, value, section_id)
    VALUES
    ('ca646ef8-67ac-4366-97c3-5d1865038766','glass_background_image_url', '', '25d135a0-77f1-49d8-bec9-ed93261ab9f5'),
    ('dfece773-7a2b-48bc-8ed7-98593857f9d9','glass_card_one_title', '', '25d135a0-77f1-49d8-bec9-ed93261ab9f5'),
    ('02b1adda-0960-4461-a17c-7500be398898','glass_card_one_image_url', '', '25d135a0-77f1-49d8-bec9-ed93261ab9f5'),
    ('073c7cb6-4b9c-4b9b-9bc7-41e725d7e167','glass_card_two_text', '', '25d135a0-77f1-49d8-bec9-ed93261ab9f5'),
    ('43aaf325-b64c-464f-8596-ef85126e8810','glass_card_three_cta_text', '', '25d135a0-77f1-49d8-bec9-ed93261ab9f5'),
    ('af58eff0-8c79-4297-9f81-25ff1152829f','glass_card_three_video_url', '', '25d135a0-77f1-49d8-bec9-ed93261ab9f5');

-- PROPERTYS: TIMELINE
INSERT OR IGNORE INTO propertys (id, name, value, section_id)
    VALUES
    ('0095a7bc-22b0-4d4f-b476-ff113415eaf1','timeline_section_title', '', '0c803974-8086-401c-817f-6ab18a190c55'),
    ('b8edb194-24df-48e3-90c3-7a6ff855d7d8','timeline_events', '[]', '0c803974-8086-401c-817f-6ab18a190c55');

-- PROPERTYS: GIFT
INSERT OR IGNORE INTO propertys (id, name, value, section_id)
    VALUES
    ('f9405541-139b-45ac-80fb-05eb4670ed88','gift_teaser_text', '', '8af963f5-add4-40da-892e-1a2d9714548b'),
    ('7f56666e-7a99-4e6d-99c5-de3f08fba992','gift_interaction_instruction', '', '8af963f5-add4-40da-892e-1a2d9714548b'),
    ('cde768b0-c9cb-4079-af5b-32e199a9356b','gift_reveal_title', '', '8af963f5-add4-40da-892e-1a2d9714548b'),
    ('43340f3a-00a9-4e96-9c6f-2bfd789b53e8','gift_surprise_name', '', '8af963f5-add4-40da-892e-1a2d9714548b'),
    ('359aacdb-e398-4f82-b3e5-c88a8f182674','gift_surprise_message', '', '8af963f5-add4-40da-892e-1a2d9714548b'),
    ('e664c212-6cd4-45f6-bd01-ff51a7816e21','gift_surprise_action_url', '', '8af963f5-add4-40da-892e-1a2d9714548b'),
    ('ff1c3f1f-92e0-40a7-82c9-6b0775676e2a','gift_surprise_button_label', '', '8af963f5-add4-40da-892e-1a2d9714548b');
