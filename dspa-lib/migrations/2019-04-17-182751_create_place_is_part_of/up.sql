CREATE TABLE place_is_part_of (
    place_id int NOT NULL,
    parent_id int NOT NULL,
    PRIMARY KEY (place_id, parent_id),
    FOREIGN KEY (place_id) REFERENCES place (id),
    FOREIGN KEY (parent_id) REFERENCES place (id)
);