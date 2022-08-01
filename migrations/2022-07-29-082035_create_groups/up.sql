-- Your SQL goes here
CREATE TABLE groups (
    id varchar(36) DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
    owner_id varchar(36) NOT NULL,
    name varchar(255) NOT NULL
);