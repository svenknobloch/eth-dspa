CREATE TABLE comment (
    id int NOT NULL,
    person_id int NOT NULL,
    creation_date timestamptz NOT NULL,
    location_ip varchar NOT NULL,
    browser_used varchar NOT NULL,
    content varchar NOT NULL,
    reply_to_post_id int,
    reply_to_comment_id int,
    place_id int NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (person_id) REFERENCES person (id),
    FOREIGN KEY (place_id) REFERENCES place (id)
);