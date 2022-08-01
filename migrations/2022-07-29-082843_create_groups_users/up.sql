-- Your SQL goes here
CREATE TABLE groups_users (
    id varchar(36) DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
    user_id varchar(36) NOT NULL,
    group_id varchar(36) NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES users(id),
    CONSTRAINT fk_group FOREIGN KEY(group_id) REFERENCES groups(id)
);