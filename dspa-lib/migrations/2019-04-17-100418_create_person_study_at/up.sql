CREATE TABLE person_study_at (
    person_id int NOT NULL,
    organization_id int NOT NULL,
    class_year int NOT NULL,
    PRIMARY KEY (person_id, organization_id),
    FOREIGN KEY (person_id) REFERENCES person (id),
    FOREIGN KEY (organization_id) REFERENCES organization (id)
);