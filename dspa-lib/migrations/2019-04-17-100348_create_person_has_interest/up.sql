CREATE TABLE person_has_interest (
    person_id int NOT NULL,
    tag_id int NOT NULL,
    PRIMARY KEY (person_id, tag_id),
    FOREIGN KEY (person_id) REFERENCES person (id),
    FOREIGN KEY (tag_id) REFERENCES tag (id)
);