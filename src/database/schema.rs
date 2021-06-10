table! {
    repositories (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        path -> Text,
        group_uuid -> Nullable<Binary>,
    }
}

table! {
    repository_groups (uuid) {
        uuid -> Binary,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(repositories, repository_groups,);
