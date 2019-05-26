CREATE TABLE tag_class_is_subclass_of (
    tag_class_id int NOT NULL,
    parent_id int NOT NULL,
    PRIMARY KEY (tag_class_id, parent_id),
    FOREIGN KEY (tag_class_id) REFERENCES tag_class (id),
    FOREIGN KEY (parent_id) REFERENCES tag_class (id)
);