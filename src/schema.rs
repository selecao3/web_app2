table! {
    creater (id) {
        id -> Int4,
        account -> Varchar,
        mail_address -> Varchar,
        password -> Varchar,
        regulation -> Bool,
    }
}

table! {
    post_img (id) {
        id -> Int4,
        account -> Varchar,
        title -> Varchar,
        body -> Text,
        img_url_1 -> Nullable<Text>,
        regulation -> Bool,
    }
}

table! {
    profile (id) {
        id -> Int4,
        name -> Varchar,
        account -> Varchar,
        profile_text -> Text,
        profile_img -> Text,
        regulation -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    creater,
    post_img,
    profile,
);
