CREATE TABLE person_knows (
    person_id int NOT NULL,
    acquaintance_id int NOT NULL,
    PRIMARY KEY (person_id, acquaintance_id),
    FOREIGN KEY (person_id) REFERENCES person (id),
    FOREIGN KEY (acquaintance_id) REFERENCES person (id)
);