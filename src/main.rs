#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate multipart;
extern crate formdata;
#[macro_use] extern crate diesel;
extern crate regex;
extern crate bcrypt;


use image::static_rocket_route_info_for_multipart_upload;
use signin::static_rocket_route_info_for_signup_post;
use creater_setting::static_rocket_route_info_for_multipart_user_setting;

use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket_contrib::Template;

use diesel::pg::PgConnection;


mod image;
mod db;
mod signin;
mod creater_setting;
mod schema;


#[derive(Serialize)]
struct TemplateRenderTest{
    name: String,

}
//テンプレートファイルに渡すstruct

//getメソッド群
#[get("/")]
fn home() -> Template{
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("news", &context)
}


#[get("/about_me")]              // <- route attribute
fn about_me() -> Template {  // <- request handler
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("about_me", &context)
}

#[get("/signup")]              // <- route attribute
fn signup() -> Template {  // <- request handler
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("sign_up", &context)
}
#[get("/login")]              // <- route attribute
fn login() -> Template {  // <- request handler
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("login", &context)
}



use rocket::request::FromForm;


/*#[get("/creater/account")]              // <- route attribute
fn user(connection: db::Connection, cookies:Cookies) -> Template {  // <- request handler
    Template::render("profile",Context::row(&connection, cookies))
}*/
#[get("/creater/account/<account>")]              // <- route attribute
fn user(connection: db::Connection, cookies:Cookies, account:String) -> Template {  // <- request handler
    if cookies.get("account").unwrap().value() == account.as_str() {
        return Template::render("profile",Context::row(&connection, cookies))
    }else {
        return Template::render("profile",Context::account_row(&connection, account))
    }

}
#[get("/creater/account/news")]              // <- route attribute
fn news(connection: db::Connection, cookies:Cookies) -> Template {  // <- request handler
    Template::render("news_creater",Context::row(&connection, cookies))
}

//getメソッド群終わり



use rocket::http::Cookies;

use rocket::http::Cookie;
use rocket::request::Form;


//postメソッド群
//postアトリビュートのurlは、formのURIに対応している。




//staticファイルを伝えるメソッド
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/<path..>", rank = 6)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
#[get("/creater/<path..>", rank = 5)]
//creater/hogehogeにstaticディレクトリを適用する
fn creater_static(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
#[get("/creater/account/<path..>", rank = 4)]
//creater/hogehogeにstaticディレクトリを適用する
fn profile_static(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
//staticファイルを伝えるメソッド終わり



use diesel::prelude::*;



#[derive(Debug,Serialize)]
struct Context{
    post_img: Vec<image::PostImg>,
    profile: Vec<creater_setting::Profile>
}


use rocket::response::Flash;


use std::env;
use std::io;
use rocket::Data;


use std::io::Read;
use std::fs;
use std::fs::File;

use std::io::Write;

extern crate rocket_static_fs;

use rocket::http::hyper::header::Headers;


#[get("/<path..>", rank = 7)]
//creater/hogehogeにstaticディレクトリを適用する
fn files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/post_image").join(path)).ok()
}



impl Context{
    fn row(connection: &db::Connection, cookies:Cookies) -> Context{
        //この関数でstructのメンバに値が渡される。＝＞"cookieのaccount"由来のものしか出さない。
        //つまり、account="hoge"で画像をpostしても、account="hage"のページでは表示されない。
        let aaa = cookies.get("account");
        Context{
            post_img: image::read_post_img(connection, aaa.clone()),
            profile:creater_setting::read_profile(&connection, aaa.clone())
        }
    }
    fn account_row(connection: &db::Connection, account:String) -> Context{
        //この関数でstructのメンバに値が渡される。＝＞"cookieのaccount"由来のものしか出さない。
        //つまり、account="hoge"で画像をpostしても、account="hage"のページでは表示されない。
        Context{
            post_img: image::read_post_img_normal(connection, account.clone()),
            profile:creater_setting::read_profile_normal(&connection, account.clone())
        }
    }
}


#[get("/creater/account/new")]
fn user_setting(connection: db::Connection, cookies:Cookies) -> Template {
    Template::render("creater_setting", Context::row(&connection, cookies))
}


/*#[get("/images")]              // <- route attribute
fn images(connection: db::Connection) -> Template {  // <- request handler
    Template::render("gallary", Context::row_gallary(&connection))
}*/





/*#[get("/creater/<account>")]              // <- route attribute
fn creater(account: User, connection: db::Connection) -> Template {  // <- request handler
    Template::render("creater_1", Context::row(&connection))
}*/

#[get("/creater")]              // <- route attribute
fn creater(connection: db::Connection) -> Template {  // <- request handler
    Template::render("creaters", ProfileContext::row(&connection))
}

//profile郡
#[derive(Debug,Serialize)]
struct ProfileContext{
    profile: Vec<creater_setting::Profile>
}

impl ProfileContext{
    fn row(connection: &db::Connection) -> ProfileContext{
        ProfileContext{
            profile:creater_setting::read_profiles_all(&connection)
        }
    }
}


fn main() {
    rocket::ignite()
        .mount("/", routes![
home,about_me,signup,login,signup_post,multipart_user_setting,
all,creater_static,files,creater,user_setting,profile_static,user,news
])
        .mount("/creater/account/post/", routes![multipart_upload])
        .manage(db::connect())
        .attach(Template::fairing())

        .launch();
}