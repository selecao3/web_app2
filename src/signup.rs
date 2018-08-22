use rocket::request::Form;
use rocket::response::Flash;

use bcrypt;

use rocket::response::Redirect;

use db;
use creater_setting;

use regex::Regex;


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

#[post("/creater/profile", data = "<user>")]
//signup.html.teraからわたされたSignup structをDBへ取り込み、なおかつaccountの値をcookieに追加して、creater_setting.html.teraへリダイレクトする
fn signup_post(cookies: Cookies, user: Form<SignupForm>, connection: db::Connection) -> Flash<Redirect>{
    let t = user.into_inner();
    let t_clone = t.clone();
    if check_account(&t_clone) != true {
        println!("check_account");
        Flash::error(Redirect::to("/signup"), "アカウントは半角英数字で入力してください。")
    }else {
        if check_double(&connection, &t_clone) == 0 {
            insert(t_clone,&connection, cookies);
            Flash::success(Redirect::to("/creater/account/new"), "")
            //creater編集画面へ
        } else {
            Flash::error(Redirect::to("/signup"), "そのアカウントは存在しています。")
        }
    }

}


use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;

use schema::creater::columns::mail_address as mail;

fn check_account(signup:&SignupForm) -> bool{
    //アカウントが半角英数字かどうか
    let re = Regex::new("^[a-zA-Z0-9]+$").unwrap();
    re.is_match(signup.account.as_str())
}

fn check_double(conn: &PgConnection, signup:&SignupForm) -> usize{
    //acountがユニークなものならtrue
    let account:usize = match self::all_creater
        .filter(creater::account.eq(signup.account.as_str())
            .or(creater::mail_address.eq(signup.mail_address.as_str())))
        .execute(conn){
        Ok(p) => p,
        Err(_) => 0
        //１行もダブっていない
    };
    account
}

fn insert(signupform:SignupForm, conn: &PgConnection,mut cookies:Cookies) -> bool{
        let t = Signup{
            id: None,
            account: signupform.account,
            mail_address: signupform.mail_address,
            password: bcrypt::hash(signupform.password.trim(), bcrypt::DEFAULT_COST).unwrap(),
            regulation: false
        };
        let cookie_account = Cookie::new("account",t.clone().account);
        cookies.add_private(cookie_account.clone());

        creater_setting::insert(conn,cookies);
        diesel::insert_into(creater::table).values(&t).execute(conn).is_ok()
        //account or mail_addressがDB上で同じだとFalse
}
