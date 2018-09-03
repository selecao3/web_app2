use rocket::request::Form;
use rocket::response::Flash;

use bcrypt;

use rocket::response::Redirect;

use db;
use signup::Signup;
use signup;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use time;

use schema::creater::columns::password;
use schema::creater;
use schema::creater::dsl::{creater as all_creater};


#[derive(FromForm, Clone)]
struct SigninForm{
    account_flag: String,
    password_flag: String,
}

use rocket::http::{Cookies,Cookie};

#[post("/creater/profile/signin", data = "<user>")]
//signup.html.teraからわたされたSignup structをDBへ取り込み、なおかつaccountの値をcookieに追加して、creater_setting.html.teraへリダイレクトする
fn signin_post(mut cookies: Cookies, user: Form<SigninForm>, connection: db::Connection) -> Flash<Redirect>{
    let signupform = user.into_inner();

    if signupform.check(&connection){
        let mut cookie = Cookie::new("account",signupform.account_flag.clone());
        let mut now = time::now();
        now.tm_year += 1;
        &cookie.set_expires(now);
        cookies.add_private(cookie);
        return Flash::success(Redirect::to(format!("/creater/account/{}",signupform.account_flag).as_str()), "")
    }else {
        return Flash::error(Redirect::to("/login"), "アカウントまたはパスワードが間違っています。")
    }

}


impl SigninForm{
    fn check(&self, conn: &PgConnection) -> bool {
        let pass:String = match signup::all_creater
            //型をStringとしないと、&str型になる。
            //返り値はString型であるにもかかわらず、変数は&str型なので一致せずにエラー。
            .filter(signup::creater::account.eq(self.account_flag.as_str()))
            .select(password)
            .first(conn){
            Ok(p) => p,
            Err(_) => "".to_string()
        };
        if pass == "" {
            //入力されたaccountの値が存在しないものだった場合
            return false
        }
        bcrypt::verify(self.password_flag.as_str(), pass.as_str()).unwrap()
    }
}


