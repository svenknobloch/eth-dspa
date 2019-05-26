CREATE TABLE tag_has_type (
    tag_id int NOT NULL,
    tag_class_id int NOT NULL,
    PRIMARY KEY (tag_id, tag_class_id),
    FOREIGN KEY (tag_id) REFERENCES tag (id),
    FOREIGN KEY (tag_class_id) REFERENCES tag_class (id)
);