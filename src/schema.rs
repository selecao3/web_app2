table! {
    post_img (id) {
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
        img_url_1 -> Nullable<Text>,
        img_url_2 -> Nullable<Text>,
        img_url_3 -> Nullable<Text>,
        img_url_4 -> Nullable<Text>,
        regulation -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    post_img,
    posts,
);
