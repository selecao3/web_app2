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

use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket_contrib::Template;

use diesel::pg::PgConnection;


mod image;
mod db;
mod signin;


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

/*#[get("/")]
fn news() -> Template{
    let context = TemplateRenderTest{
        name: "name".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("news", &context)
}*/



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

/*#[get("/creater/<user>", rank = 2)]              // <- route attribute
fn admin(user: &RawStr) -> String {  // <- request handler
    format!("{}の個人ページ", user.as_str())
}*/
/*#[get("/creater/account")]              // <- route attribute
fn user() -> Template {  // <- request handler
   let context = TemplateRenderTest02{
        text: "hogehoge".to_string()
        //nameという文字列がHome.html.teraの{{name}}に渡される
    };
    Template::render("creater_1", &context)
}*/
/*#[get("/creater/account", rank = 3)]
fn redirect_admin() -> Redirect {
    Redirect::to("/login")
}*/
//getメソッド群終わり


use rocket::http::{Cookie, Cookies};
use rocket::request::Form;


#[derive(Serialize)]
struct TemplateRenderTest02{
    text: String,
    //writting text in user's textarea.
    //you can use in template.

}
//postメソッド群
//postアトリビュートのurlは、formのURIに対応している。


/*#[post("/post/sign_in", data = "<sign_in>")]
fn signin(sign_in: Form<Profile>) -> String{
}*/

/*#[post("/post/sign_up", data = "<sign_up>")]
fn regist(sign_up: Form<User>) -> String{

}*/
/*#[post("/post/logout", data = "<logout>")]
fn logout(logout: Form<Profile>) -> String{

}*/


//staticファイルを伝えるメソッド
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
#[get("/creater/<path..>", rank = 4)]
//creater/hogehogeにstaticディレクトリを適用する
fn creater_static(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
//staticファイルを伝えるメソッド終わり

/*//databases
#[macro_use] extern crate diesel;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use diesel::QueryDsl;


// An alias to the type for a pool of Diesel Mysql Connection
type PgPool = Pool<ConnectionManager<PgConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &str = env!("DATABASE_URL");

/// Initialize the database pool.
fn connect() -> PgPool{
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    Pool::new(manager).expect("Failed to create pool")
}

// Connection request guard type: a wrapper around an r2d2 pooled connection.
struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &Connection as an &MysqlConnection.
impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}*/


use diesel::prelude::*;


mod schema {
    table! {
    posts (id) {
        id -> Nullable<Int4>,
        account -> Varchar,
        title -> Varchar,
        body -> Nullable<Text>,
        img_url_1 -> Nullable<Text>,
        img_url_2 -> Nullable<Text>,
        img_url_3 -> Nullable<Text>,
        img_url_4 -> Nullable<Text>,
        regulation -> Bool,
    }
}
    table! {
    profile (id) {
        id -> Nullable<Int4>,
        name -> Varchar,
        account -> Varchar,
        profile_text -> Text,
        profile_img -> Text,
        regulation -> Bool,
    }
}
}
/*    table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> VarChar,
        //published -> Datetime,
        body -> Text,
        regulation -> Bool,
    }
}
}*/


/*use self::schema::posts;
use self::schema::posts::dsl::{posts as all_posts, regulation as post_regulation};*/

/*#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "posts"]
struct Post {
    id: Option<i32>,
    account: String,
    title: String,
    body: Option<String>,
    img_url_1: Option<String>,
    img_url_2: Option<String>,
    img_url_3: Option<String>,
    img_url_4: Option<String>,
    regulation: bool
}

#[derive(FromForm)]
struct PostForm{
    title: String,
    body: Option<String>,
    img_url_1: Option<String>,
    img_url_2: Option<String>,
    img_url_3: Option<String>,
    img_url_4: Option<String>,
    regulation: bool,
    *//*    regulation: bool,*//*
}
#[derive(FromForm)]
struct GallaryForm{
    img_url: String,
    user_url: String,
    regulation: bool
    *//*    regulation: bool,*//*
}*/
/*
#[derive(Serialize, Queryable, Debug,Clone)]
#[table_name = "user"]
struct User{
    id: Option<i32>,
    name: String,
    mail_address: String,
    account: String,
    password: String,
}
*/

#[derive(Debug,Serialize)]
struct Context{
    post_img: Vec<image::PostImg>
}
/*#[derive(Debug,Serialize)]
struct ContextGallary{
    post_img: Vec<image::PostImg>
}*/

/*fn read(connection: &PgConnection) -> Vec<Post> {
    //postsテーブルからデータを読み取る。
    all_posts
        .order(posts::id.desc())
        .load::<Post>(connection)
        .expect("error")
}

fn insert(postform:PostForm, conn: &PgConnection) -> bool{
    let t = Post{
        id: None,
        account: "root".to_string(),
        title: postform.title,
        body: postform.body,
        img_url_1: postform.img_url_1,
        img_url_2: postform.img_url_2,
        img_url_3: postform.img_url_3,
        img_url_4: postform.img_url_4,
        //保存したimg_urlをどうにかしてPost structへ・・・
        regulation: false
    };
    *//*    println!("insert method");
        println!("{}&{}",t.title,t.body);

        let a = diesel::insert_into(posts::table).values(&t).execute(conn).unwrap();

        //上の一行をコメントアウトすると一度のPOSTで二つ同じものをinsertすることになる（バグ）

        println!("{:?}",a);*//*
    diesel::insert_into(posts::table).values(&t).execute(conn).is_ok()
}*/
use rocket::response::Flash;

/*#[post("/form_hoge", data = "<toukou_form>")]
fn new(toukou_form: Form<PostForm>, connection: db::Connection) -> Flash<Redirect>{
    let t = toukou_form.into_inner();

    println!("postを通ってます。");
    if insert(t,&connection) {
        println!("成功してる");

        Flash::success(Redirect::to("/creater/account/post/form_img"), "成功してる")
    } else {
        println!("失敗");
        Flash::error(Redirect::to("/creater/account"), "失敗した。")
    }
}*/


/*#[post("/form", data = "<toukou>")]
fn article(toukou: Form<PostForm>, connection: db::Connection) -> Flash<Redirect>{
    let t = toukou.into_inner();

    println!("post");
    if insert(t,&connection) {
        println!("成功");
        Flash::success(Redirect::to("/creater/account"), "成功してる")
    } else {
        println!("失敗");
        Flash::error(Redirect::to("/creater/account"), "失敗した。")
    }
}*/


use std::env;
use std::io;
use rocket::Data;


use std::io::Read;
use std::fs;
use std::fs::File;

use std::io::Write;

extern crate rocket_static_fs;

use rocket::http::hyper::header::Headers;


#[get("/<path..>", rank = 6)]
//creater/hogehogeにstaticディレクトリを適用する
fn files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/post_image").join(path)).ok()
}


impl Context{
    fn row(connection: &db::Connection) -> Context{
        Context{
            post_img: image::read_post_img(connection)
        }
    }
    fn row_gallary(connection: &db::Connection) -> Context{
        Context{
            post_img: image::read_gallary(connection)
        }
    }
}


//fn raw(conn: &Connection) -> Vec<Post>{
//    post: Context::read()
//}


#[get("/creater/account")]
fn hoge(connection: db::Connection) -> Template {
    println!("get中");
    Template::render("creater_1", Context::row(&connection))
}

#[get("/images")]              // <- route attribute
fn images(connection: db::Connection) -> Template {  // <- request handler
    Template::render("gallary", Context::row_gallary(&connection))
}

/*#[get("/creater/<account>")]              // <- route attribute
fn creater(account: User, connection: db::Connection) -> Template {  // <- request handler
    Template::render("creater_1", Context::row(&connection))
}*/

#[get("/creater")]              // <- route attribute
fn creater(connection: db::Connection) -> Template {  // <- request handler
    Template::render("creaters", Context::row_gallary(&connection))
}







use self::schema::profile;
use self::schema::profile::dsl::{profile as all_profile , regulation as profile_regulation};

#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "profile"]
struct Profile{
    id: Option<i32>,
    name: String,
    account: String,
    profile_text: String,
    profile_img : String,
    regulation: bool
}

#[derive(FromForm)]
struct ProfileForm{
    name: String,
    account: String,
    profile: String,
    profile_img: String,
    /*    regulation: bool,*/
}

//profile郡
#[derive(Debug,Serialize)]
struct ProfileContext{
    profile: Vec<Profile>
}

impl ProfileContext{
    fn row(connection: &db::Connection) -> ProfileContext{
        ProfileContext{
            profile: read_profile(connection)
        }
    }
}
fn read_profile(connection: &PgConnection) -> Vec<Profile> {
    //postsテーブルからデータを読み取る。
    all_profile
        //accountが◯◯のものを取り出す
        .order(profile::id.desc())
        .load::<Profile>(connection)
        .expect("error")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
home,images,about_me,signup,login,signup_post,
all,creater_static,hoge,files,creater
])
        .mount("/creater/account/post/", routes![multipart_upload])
        .manage(db::connect())
        .attach(Template::fairing())

        .launch();
}