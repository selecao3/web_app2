use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use image;
use db;

use image::schema::post_img;
use signin::schema::creater;
use schema::profile;



fn join_table(connection: &PgConnection){
    joinable!(post_img -> profile (id));
    allow_tables_to_appear_in_same_query!(post_img, profile);

    let data = post_img
        .inner_join(profile)
        .select((post_img::title, post_img::body, post_img::img_url_1, profile::profile_text, profile::profile_img, profile::name))
        .load(connection);
}
