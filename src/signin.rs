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

use schema::creater;
use schema::creater::dsl::{creater as all_creater};

#[derive(FromForm, Clone)]
struct SigninForm{
    account_flag: String,
    password_flag: String,
}

use rocket::http::{Cookies,Cookie};
use std::collections::HashMap;

#[post("/creater/profile/signin", data = "<user>")]
//signup.html.teraからわたされたSignup structをDBへ取り込み、なおかつaccountの値をcookieに追加して、creater_setting.html.teraへリダイレクトする
fn signin_post(mut cookies: Cookies, user: Form<SigninForm>, connection: db::Connection) -> Flash<Redirect>{
    let t = user.into_inner();

    println!("post");
    if check(&connection, &cookies, t.clone()){
        let mut cookie = Cookie::new("account",t.account_flag.clone());
        cookies.add_private(cookie);
        println!("成功");
        return Flash::success(Redirect::to(format!("/creater/account/{}",t.account_flag).as_str()), "成功してる")
    }else {
        return Flash::error(Redirect::to("/login"), "アカウントまたはパスワードが間違っています。")
    }

}

use signup::Signup;
use signup;

use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;

use schema::profile::columns::account;
use schema::creater::columns::password;

fn check(conn: &PgConnection, mut cookies: &Cookies, signin:SigninForm) -> bool {
    let pass:String = match signup::all_creater
        //型をStringとしないと、&str型になる。
        //返り値はString型であるにもかかわらず、変数は&str型なので一致せずにエラー。
        .filter(signup::creater::account.eq(signin.account_flag.as_str()))
        .select(password)
        .first(conn){
        Ok(p) => p,
        Err(e) => "".to_string()
    };
    println!("{}",pass);
    println!("{}",signin.password_flag.as_str());
    match bcrypt::verify(signin.password_flag.as_str(), pass.as_str()){
        Ok(pw) => true,
        Err(e) => false
    }
}
pub fn read_user(connection: &PgConnection) -> Vec<Signup> {
    //postsテーブルからデータを読み取る。
    all_creater
        .order(creater::id.desc())
        .load::<Signup>(connection)
        .expect("error")
}
