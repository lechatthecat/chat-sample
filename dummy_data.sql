-- services テーブルにダミーデータ
INSERT INTO
    services (name)
VALUES
    ('Eat-in'),
    ('Take-out'),
    ('Reservation');
-- users テーブルにダミーデータ
INSERT INTO
    users (name, password)
VALUES
    ('Alice', 123),
    ('Bob', 123),
    ('Charlie', 123);
-- rooms テーブルにダミーデータ
INSERT INTO
    rooms (name)
VALUES
    ('Room A'),
    ('Room B'),
    ('Room C');
-- room_users テーブルにダミーデータ
-- 例：room_id 1,2,3にuser_id 1,2,3を割り当てるパターン（組み合わせ自由です）
INSERT INTO
    room_users (room_id, user_id)
VALUES
    (1, 1),
    (2, 2),
    (3, 3);