CREATE TABLE post (
    id int NOT NULL,
    person_id int NOT NULL,
    creation_date timestamptz NOT NULL,
    image_file varchar,
    location_ip varchar NOT NULL,
    browser_used varchar NOT NULL,
    language varchar,
    content varchar,
    tags int[] NOT NULL,
    forum_id int NOT NULL,
    place_id int NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (person_id) REFERENCES person (id),
    FOREIGN KEY (forum_id) REFERENCES forum (id),
    FOREIGN KEY (place_id) REFERENCES place (id)
);