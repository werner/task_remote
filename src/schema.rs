table! {
    languages (id) {
        id -> Integer,
        name -> Text,
        value -> Text,
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
    tasks,
);
