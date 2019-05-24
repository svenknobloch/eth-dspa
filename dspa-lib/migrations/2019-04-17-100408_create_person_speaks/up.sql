CREATE TABLE person_speaks (
    person_id int NOT NULL,
    language varchar NOT NULL,
    PRIMARY KEY (person_id, language),
    FOREIGN KEY (person_id) REFERENCES person (id)
);