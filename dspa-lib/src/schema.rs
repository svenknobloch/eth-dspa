table! {
    comment (id) {
        id -> Int4,
        person_id -> Int4,
        creation_date -> Timestamptz,
        location_ip -> Varchar,
        browser_used -> Varchar,
        content -> Varchar,
        reply_to_post_id -> Nullable<Int4>,
        reply_to_comment_id -> Nullable<Int4>,
        place_id -> Int4,
    }
}

table! {
    forum (id) {
        id -> Int4,
        title -> Varchar,
        creation_date -> Timestamptz,
    }
}

table! {
    forum_has_member (forum_id, person_id) {
        forum_id -> Int4,
        person_id -> Int4,
        join_date -> Timestamptz,
    }
}

table! {
    forum_has_moderator (forum_id, person_id) {
        forum_id -> Int4,
        person_id -> Int4,
    }
}

table! {
    forum_has_tag (forum_id, tag_id) {
        forum_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    like_ (person_id, post_id) {
        person_id -> Int4,
        post_id -> Int4,
        creation_date -> Timestamptz,
    }
}

table! {
    organization (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        name -> Varchar,
        url -> Varchar,
    }
}

table! {
    organization_is_located_in (organization_id, place_id) {
        organization_id -> Int4,
        place_id -> Int4,
    }
}

table! {
    person (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        gender -> Varchar,
        birthday -> Date,
        creation_date -> Timestamptz,
        location_ip -> Varchar,
        browser_used -> Varchar,
    }
}

table! {
    person_email (person_id, email) {
        person_id -> Int4,
        email -> Varchar,
    }
}

table! {
    person_has_interest (person_id, tag_id) {
        person_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    person_is_located_in (person_id, place_id) {
        person_id -> Int4,
        place_id -> Int4,
    }
}

table! {
    person_knows (person_id, acquaintance_id) {
        person_id -> Int4,
        acquaintance_id -> Int4,
    }
}

table! {
    person_speaks (person_id, language) {
        person_id -> Int4,
        language -> Varchar,
    }
}

table! {
    person_study_at (person_id, organization_id) {
        person_id -> Int4,
        organization_id -> Int4,
        class_year -> Int4,
    }
}

table! {
    person_work_at (person_id, organization_id) {
        person_id -> Int4,
        organization_id -> Int4,
        work_from -> Int4,
    }
}

table! {
    place (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    place_is_part_of (place_id, parent_id) {
        place_id -> Int4,
        parent_id -> Int4,
    }
}

table! {
    post (id) {
        id -> Int4,
        person_id -> Int4,
        creation_date -> Timestamptz,
        image_file -> Nullable<Varchar>,
        location_ip -> Varchar,
        browser_used -> Varchar,
        language -> Nullable<Varchar>,
        content -> Nullable<Varchar>,
        tags -> Array<Int4>,
        forum_id -> Int4,
        place_id -> Int4,
    }
}

table! {
    tag (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
    }
}

table! {
    tag_class (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
    }
}

table! {
    tag_class_is_subclass_of (tag_class_id, parent_id) {
        tag_class_id -> Int4,
        parent_id -> Int4,
    }
}

table! {
    tag_has_type (tag_id, tag_class_id) {
        tag_id -> Int4,
        tag_class_id -> Int4,
    }
}

joinable!(comment -> person (person_id));
joinable!(comment -> place (place_id));
joinable!(forum_has_member -> forum (forum_id));
joinable!(forum_has_member -> person (person_id));
joinable!(forum_has_moderator -> forum (forum_id));
joinable!(forum_has_moderator -> person (person_id));
joinable!(forum_has_tag -> forum (forum_id));
joinable!(forum_has_tag -> tag (tag_id));
joinable!(like_ -> person (person_id));
joinable!(organization_is_located_in -> organization (organization_id));
joinable!(organization_is_located_in -> place (place_id));
joinable!(person_email -> person (person_id));
joinable!(person_has_interest -> person (person_id));
joinable!(person_has_interest -> tag (tag_id));
joinable!(person_is_located_in -> person (person_id));
joinable!(person_is_located_in -> place (place_id));
joinable!(person_speaks -> person (person_id));
joinable!(person_study_at -> organization (organization_id));
joinable!(person_study_at -> person (person_id));
joinable!(person_work_at -> organization (organization_id));
joinable!(person_work_at -> person (person_id));
joinable!(post -> forum (forum_id));
joinable!(post -> person (person_id));
joinable!(post -> place (place_id));
joinable!(tag_has_type -> tag (tag_id));
joinable!(tag_has_type -> tag_class (tag_class_id));

allow_tables_to_appear_in_same_query!(
    comment,
    forum,
    forum_has_member,
    forum_has_moderator,
    forum_has_tag,
    like_,
    organization,
    organization_is_located_in,
    person,
    person_email,
    person_has_interest,
    person_is_located_in,
    person_knows,
    person_speaks,
    person_study_at,
    person_work_at,
    place,
    place_is_part_of,
    post,
    tag,
    tag_class,
    tag_class_is_subclass_of,
    tag_has_type,
);
