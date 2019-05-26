CREATE TABLE person (
    id int NOT NULL,
    first_name varchar NOT NULL,
    last_name varchar NOT NULL,
    gender varchar NOT NULL,
    birthday date NOT NULL,
    creation_date timestamptz NOT NULL,
    location_ip varchar NOT NULL,
    browser_used varchar NOT NULL,
    PRIMARY KEY (id)
);