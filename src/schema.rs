table! {
    tasks (id) {
        id -> Integer,
        title -> Text,
        pre_hook -> Nullable<Text>,
        code -> Text,
        post_hook -> Nullable<Text>,
        language -> Nullable<Text>,
    }
}
