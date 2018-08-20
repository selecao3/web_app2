use rocket::request::Form;
use rocket::response::Flash;

use bcrypt;

use rocket::response::Redirect;

use db;

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
    let t = user.into_inner();

    println!("post");
    if check(&connection,  t.clone()){
        let cookie = Cookie::new("account",t.account_flag.clone());
        cookies.add_private(cookie);
        println!("成功");
        return Flash::success(Redirect::to(format!("/creater/account/{}",t.account_flag).as_str()), "")
    }else {
        return Flash::error(Redirect::to("/login"), "アカウントまたはパスワードが間違っています。")
    }

}

use signup::Signup;
use signup;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use schema::creater::columns::password;

fn check(conn: &PgConnection, signin:SigninForm) -> bool {
    let pass:String = match signup::all_creater
        //型をStringとしないと、&str型になる。
        //返り値はString型であるにもかかわらず、変数は&str型なので一致せずにエラー。
        .filter(signup::creater::account.eq(signin.account_flag.as_str()))
        .select(password)
        .first(conn){
        Ok(p) => p,
        Err(_) => "".to_string()
    };
    if pass == "" {
       //入力されたaccountの値が存在しないものだった場合
        return false
    }
    bcrypt::verify(signin.password_flag.as_str(), pass.as_str()).unwrap()
}
