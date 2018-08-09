use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;
use rocket::request::Form;
use rocket::response::Flash;

use std::io::{self, Cursor, Write};
use rocket::response::Redirect;
use std::env;

use db;



use rocket::http::{Cookies,Cookie};

#[get("/creater/profile/signout")]
//signup.html.teraからわたされたSignup structをDBへ取り込み、なおかつaccountの値をcookieに追加して、creater_setting.html.teraへリダイレクトする
fn signout_post(mut cookies: Cookies) -> Flash<Redirect>{
    let account = cookies.get_private("account").unwrap();
    if let Some(x) = account.path(){
        println!("{}", x);
    }else {
        println!("２０行目しくじってる");
    }
    cookies.remove_private(account);
    Flash::success(Redirect::to("/"), "成功してる")
}
