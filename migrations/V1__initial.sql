CREATE TABLE users (
    id INT NOT NULL AUTO_INCREMENT,
    name varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    login_hash varchar(255),
    PRIMARY KEY (id)
) CHARACTER SET utf8mb4;