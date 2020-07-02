-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    login_session VARCHAR NOT NULL DEFAULT ''
);

CREATE TABLE persons (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    -- posix seconds
    birthdate BIGINT NOT NULL,
    user_id INT NOT NULL REFERENCES users (id)
);

CREATE TABLE login_history (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT NOT NULL REFERENCES users (id),
    login_timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);