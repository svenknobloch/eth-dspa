CREATE TABLE forum_has_tag (
    forum_id int NOT NULL,
    tag_id int NOT NULL,
    PRIMARY KEY (forum_id, tag_id),
    FOREIGN KEY (forum_id) REFERENCES forum (id),
    FOREIGN KEY (tag_id) REFERENCES tag (id)
);