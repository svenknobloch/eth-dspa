CREATE TABLE forum_has_member (
    forum_id int NOT NULL,
    person_id int NOT NULL,
    join_date timestamptz NOT NULL,
    PRIMARY KEY (forum_id, person_id),
    FOREIGN KEY (forum_id) REFERENCES forum (id),
    FOREIGN KEY (person_id) REFERENCES person (id)
);