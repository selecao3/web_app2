use rocket::request::Form;
use rocket::response::Flash;

use bcrypt;

use rocket::response::Redirect;

use db;
use creater_setting;

use regex::Regex;
use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;

use schema::creater::columns::mail_address as mail;
use rocket::http::{Cookies,Cookie};

pub use schema::creater;
pub use schema::creater::dsl::{creater as all_creater};

#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "creater"]
pub struct Signup{
    id: Option<i32>,
    account: String,
    mail_address: String,
    password: String,
}

#[derive(FromForm, Clone)]
struct SignupForm{
    account: String,
    mail_address: String,
    password: String,
}



#[post("/creater/profile", data = "<user>")]
//signup.html.teraからわたされたSignup structをDBへ取り込み、なおかつaccountの値をcookieに追加して、creater_setting.html.teraへリダイレクトする
fn signup_post(cookies: Cookies, user: Form<SignupForm>, connection: db::Connection) -> Flash<Redirect>{
    let t = user.into_inner();
    let signupform = t.clone();
    if signupform.check_account() != true {
        println!("check_account");
        Flash::error(Redirect::to("/signup"), "アカウントは半角英数字で入力してください。")
    }else {
        if signupform.check_double(&connection) == 0 {
            signupform.insert(&connection, cookies);
            Flash::success(Redirect::to("/creater/account/new"), "")
            //creater編集画面へ
        } else {
            Flash::error(Redirect::to("/signup"), "そのアカウントは存在しています。")
        }
    }

}




impl SignupForm{
    fn check_account(&self) -> bool{
        //アカウントが半角英数字かどうか
        let re = Regex::new("^[a-zA-Z0-9]+$").unwrap();
        re.is_match(&self.account.as_str())
    }

    fn check_double(&self, conn: &PgConnection) -> usize{
        //acountがユニークなものならtrue
        let account:usize = match self::all_creater
            .filter(creater::account.eq(&self.account.as_str())
                .or(creater::mail_address.eq(&self.mail_address.as_str())))
            .execute(conn){
            Ok(p) => p,
            Err(_) => 0
            //１行もダブっていない
        };
        account
    }

    fn insert(self, conn: &PgConnection,mut cookies:Cookies) -> bool{
        let t = Signup{
            id: None,
            account: self.account,
            mail_address: self.mail_address,
            password: bcrypt::hash(self.password.trim(), bcrypt::DEFAULT_COST).unwrap(),
        };
        let cookie_account = Cookie::new("account",t.clone().account);
        cookies.add_private(cookie_account.clone());

        creater_setting::insert(conn,cookies);
        diesel::insert_into(creater::table).values(&t).execute(conn).is_ok()
        //account or mail_addressがDB上で同じだとFalse
    }
}

