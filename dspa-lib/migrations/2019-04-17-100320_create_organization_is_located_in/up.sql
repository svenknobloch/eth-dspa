CREATE TABLE organization_is_located_in (
    organization_id int NOT NULL,
    place_id int NOT NULL,
    PRIMARY KEY (organization_id, place_id),
    FOREIGN KEY (organization_id) REFERENCES organization (id),
    FOREIGN KEY (place_id) REFERENCES place (id)
);