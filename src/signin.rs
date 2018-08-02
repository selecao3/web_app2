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
        cookies.add(Cookie::new("account",t.account_flag));
        println!("成功");
        return Flash::success(Redirect::to("/creater/account/new"), "成功してる")
    }else {
        return Flash::error(Redirect::to("/"), "miss")
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
    let pass:String = signup::all_creater
        //型をStringとしないと、&str型になる。
        //返り値はString型であるにもかかわらず、変数は&str型なので一致せずにエラー。
        .filter(signup::creater::account.eq(signin.account_flag.as_str()))
        .select(password)
        .first(conn).unwrap();
    println!("{}",pass);
    bcrypt::verify(pass.as_str(),signin.password_flag.as_str()).unwrap()
}
/*fn insert(signupform:SignupForm, conn: &PgConnection,mut cookies:Cookies) -> bool{
    let t = Signup{
        id: None,
        account: signupform.account,
        mail_address: signupform.mail_address,
        password: bcrypt::hash(signupform.password.trim(), bcrypt::DEFAULT_COST).unwrap(),
        regulation: false
    };
    cookies.add(Cookie::new("account",t.clone().account));
    diesel::insert_into(creater::table).values(&t).execute(conn).is_ok()
    //account or mail_addressがDB上で同じだとFalse


}*/
pub fn read_user(connection: &PgConnection) -> Vec<Signup> {
    //postsテーブルからデータを読み取る。
    all_creater
        .order(creater::id.desc())
        .load::<Signup>(connection)
        .expect("error")
}
