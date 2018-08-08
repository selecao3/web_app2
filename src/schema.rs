table! {
    creater (id) {
        id -> Nullable<Int4>,
        account -> Varchar,
        mail_address -> Varchar,
        password -> Varchar,
        regulation -> Bool,
    }
}

table! {
    post_img (id) {
        id -> Nullable<Int4>,
        account -> Varchar,
        name -> Varchar,
        title -> Varchar,
        body -> Text,
        img_url_1 -> Text,
        regulation -> Bool,
    }
}

table! {
    profile (id) {
        id -> Nullable<Int4>,
        name -> Varchar,
        account -> Varchar,
        profile_text -> Text,
        profile_img -> Text,
        content01 -> Varchar,
        content02 -> Varchar,
        content03 -> Varchar,
        content04 -> Varchar,
        regulation -> Bool,
        created_day -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    creater,
    post_img,
    profile,
);

