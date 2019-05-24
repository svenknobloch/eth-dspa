CREATE TABLE forum_has_moderator (
    forum_id int NOT NULL,
    person_id int NOT NULL,
    PRIMARY KEY (forum_id, person_id),
    FOREIGN KEY (forum_id) REFERENCES forum (id),
    FOREIGN KEY (person_id) REFERENCES person (id)
);