table! {
    img (id) {
        id -> Int4,
        account -> Varchar,
        img_url_1 -> Nullable<Text>,
        regulation -> Bool,
    }
}

table! {
    posts (id) {
        id -> Int4,
        account -> Varchar,
        title -> Varchar,
        body -> Text,
        regulation -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    img,
    posts,
);
