-- Add migration script here

CREATE TABLE admin (
    admin_uuid uuid NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password_hash VARCHAR(100) NOT NULL,
    UNIQUE(admin_uuid, email)
);