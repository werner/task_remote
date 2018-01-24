table! {
    languages (id) {
        id -> Integer,
        name -> Text,
        value -> Text,
    }
}

table! {
    servers (id) {
        id -> Integer,
        user -> Text,
        domain_name -> Text,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        command -> Nullable<Text>,
        code -> Text,
        output -> Nullable<Text>,
        language -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    languages,
    servers,
    tasks,
);
