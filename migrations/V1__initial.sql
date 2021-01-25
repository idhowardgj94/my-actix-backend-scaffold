CREATE TABLE users (
    id INT NOT NULL AUTO_INCREMENT,
    name varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO users ("name", "password") VALUES (
    "howardgj94",
    "sophia0127"
);
