CREATE
OR REPLACE FUNCTION update_modified_column() RETURNS TRIGGER AS
$$
BEGIN
NEW.updated_at = now();
RETURN NEW;
END;
$$
language 'plpgsql';
-- Create restaurant tables table
DROP TABLE IF EXISTS room_users;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS services;
CREATE TABLE services (
    id INTEGER primary key generated always as identity,
    name VARCHAR(500) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER update_modified_time_services BEFORE
UPDATE
    ON services FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
CREATE TABLE users (
    id INTEGER primary key generated always as identity,
    name VARCHAR(500) NOT NULL,
    password VARCHAR(50) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER update_modified_time_users BEFORE
UPDATE
    ON users FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
CREATE TABLE rooms (
    id INTEGER primary key generated always as identity,
    name VARCHAR(500) NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER update_modified_time_rooms BEFORE
UPDATE
    ON rooms FOR EACH ROW EXECUTE PROCEDURE update_modified_column();
-- room_usersテーブル
CREATE TABLE room_users (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    room_id INTEGER NOT NULL REFERENCES rooms(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER update_modified_time_room_users BEFORE
UPDATE
    ON room_users FOR EACH ROW EXECUTE PROCEDURE update_modified_column();