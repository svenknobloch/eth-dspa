CREATE TABLE like_ (
    person_id int NOT NULL,
    post_id int NOT NULL,
    creation_date timestamptz NOT NULL,
    PRIMARY KEY (person_id, post_id),
    FOREIGN KEY (person_id) REFERENCES person (id)
);