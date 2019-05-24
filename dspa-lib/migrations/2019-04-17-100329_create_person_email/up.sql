CREATE TABLE person_email (
    person_id int NOT NULL,
    email varchar NOT NULL,
    PRIMARY KEY (person_id, email),
    FOREIGN KEY (person_id) REFERENCES person (id)
);