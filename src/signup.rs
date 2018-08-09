use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;
use rocket::request::Form;
use rocket::response::Flash;

use bcrypt;

use std::io::{self, Cursor, Write};
use rocket::response::Redirect;
use std::env;

use db;



pub use schema::creater;
pub use schema::creater::dsl::{creater as all_creater};

#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "creater"]
pub struct Signup{
    id: Option<i32>,
    account: String,
    mail_address: String,
    password: String,
    regulation: bool
}

#[derive(FromForm, Clone)]
struct SignupForm{
    account: String,
    mail_address: String,
    password: String,
}

use rocket::http::{Cookies,Cookie};
use std::collections::HashMap;

#[post("/creater/profile", data = "<user>")]
//signup.html.teraからわたされたSignup structをDBへ取り込み、なおかつaccountの値をcookieに追加して、creater_setting.html.teraへリダイレクトする
fn signup_post(mut cookies: Cookies, user: Form<SignupForm>, connection: db::Connection) -> Flash<Redirect>{
    let t = user.into_inner();
    let t_clone = t.clone();

    println!("post");
    if insert(t_clone,&connection, cookies) {
        println!("成功");

        Flash::success(Redirect::to("/creater/account/new"), "成功してる")
        //creater編集画面へ
    } else {
        Flash::error(Redirect::to("/"), "失敗した。")
    }
}


use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;
fn insert(signupform:SignupForm, conn: &PgConnection,mut cookies:Cookies) -> bool{
    let t = Signup{
        id: None,
        account: signupform.account,
        mail_address: signupform.mail_address,
        password: bcrypt::hash(signupform.password.trim(), bcrypt::DEFAULT_COST).unwrap(),
        regulation: false
    };
    let mut cookie_account = Cookie::new("account",t.clone().account);
    cookies.add_private(cookie_account.clone());
    diesel::insert_into(creater::table).values(&t).execute(conn).is_ok()
    //account or mail_addressがDB上で同じだとFalse


}
pub fn read_user(connection: &PgConnection) -> Vec<Signup> {
    //postsテーブルからデータを読み取る。
    all_creater
        .order(creater::id.desc())
        .load::<Signup>(connection)
        .expect("error")
}
