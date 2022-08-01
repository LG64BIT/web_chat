CREATE TABLE users
(
  id varchar(36) DEFAULT uuid_generate_v4() PRIMARY KEY NOT NULL,
  username varchar(255) NOT NULL ,
  "password" varchar(255) NOT NULL
);