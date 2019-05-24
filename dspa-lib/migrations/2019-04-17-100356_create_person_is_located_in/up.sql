CREATE TABLE person_is_located_in (
    person_id int NOT NULL,
    place_id int NOT NULL,
    PRIMARY KEY (person_id, place_id),
    FOREIGN KEY (person_id) REFERENCES person (id),
    FOREIGN KEY (place_id) REFERENCES place (id)
);