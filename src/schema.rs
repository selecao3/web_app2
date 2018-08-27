table! {
    creater (id) {
        id -> Nullable<Int4>,
        account -> Varchar,
        mail_address -> Varchar,
        password -> Varchar,
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
        img_url_2 -> Text,
        img_url_3 -> Text,
        img_url_4 -> Text,
        adult_check -> Varchar,
        created_day -> Varchar,
    }
}

table! {
    profile (id) {
        id -> Nullable<Int4>,
        name -> Varchar,
        account -> Varchar,
        profile_text -> Text,
        profile_img -> Text,
        created_day -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    creater,
    post_img,
    profile,
);

