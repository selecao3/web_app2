#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
#![feature(custom_attribute)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate multipart;
extern crate formdata;
#[macro_use] extern crate diesel;
extern crate bcrypt;
extern crate comrak;
extern crate chrono;
extern crate chrono_tz;
extern crate regex;
extern crate time;

use news_posts::static_rocket_route_info_for_multipart_upload;
use signup::static_rocket_route_info_for_signup_post;
use creater_setting::static_rocket_route_info_for_multipart_user_setting;
use signin::static_rocket_route_info_for_signin_post;
use signout::static_rocket_route_info_for_signout_post;

use rocket::response::Redirect;
use rocket_contrib::Template;
use rocket::response::content;




mod news_posts;
mod db;
mod signup;
mod signin;
mod creater_setting;
mod schema;
mod signout;



#[derive(Serialize)]
struct UserCookies{
    user_lisence: bool,
}
//テンプレートファイルに渡すstruct

#[catch(404)]
fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("<p>'{}'というURLは存在しません</p>
            <p>お手数ですが<a href=\"/\">ホーム</a>へお戻りください。</p>",
            req.uri()))
}


//getメソッド群
#[get("/")]
fn home(conn:db::Connection, mut cookie:Cookies) -> Template{
    if let Some(_) = cookie.get_private("account"){
        let mut context = Context::row_image(&conn, cookie);
        let context = Context{user_lisence:true, .. context};
        return Template::render("news", &context)
    }else {
        return Template::render("news", Context::row_image(&conn, cookie));
    }
}

#[get("/help")]
fn help(mut cookie:Cookies) -> Template {  // <- request handler
    if let Some(_) = cookie.get_private("account"){
        let context = UserCookies{
            user_lisence: true
        };
        return Template::render("help", &context)
    }else {
        let context = UserCookies{
            user_lisence: false
        };
        return Template::render("help", &context)
    }
}


#[get("/signup")]              // <- route attribute
fn signup(msg:Option<FlashMessage>,mut cookie:Cookies) -> Template {  // <- request handler
    let m = match msg {
        Some(m) => m.msg().to_string(),
        None => "".to_string()
    };
    println!("{}",m);

    if let Some(_) = cookie.get_private("account"){
        let context = LoginMsg{
            user_lisence: true,
            message: m
        };
        return Template::render("sign_up", &context)
    }else {
        let context = LoginMsg{
            user_lisence: false,
            message: m
        };
        return Template::render("sign_up", &context)
    }
}


#[derive(Debug,Serialize)]
struct LoginMsg{
    user_lisence:bool,
    message: String
}


use rocket::request::FlashMessage;
#[get("/login")]              // <- route attribute
fn login(msg:Option<FlashMessage>,mut cookie:Cookies) -> Template {  // <- request handler
    let m = match msg {
        Some(m) => m.msg().to_string(),
        None => "".to_string()
    };

    if let Some(_) = cookie.get_private("account"){
        let context = LoginMsg{
            user_lisence: true,
            message: m
        };
        return Template::render("signin", &context)
    }else {
        let context = LoginMsg{
            user_lisence: false,
            message: m
        };
        return Template::render("signin", &context)
    }
}

#[get("/about_me")]              // <- route attribute
fn about_me(mut cookie:Cookies) -> Template {  // <- request handler
    if let Some(_) = cookie.get_private("account"){
        let context = UserCookies{
            user_lisence: true
        };
        return Template::render("about_me", &context)
    }else {
        let context = UserCookies{
            user_lisence: false
        };
        return Template::render("about_me", &context)
    }
}




/*#[get("/creater/account")]              // <- route attribute
fn user(connection: db::Connection, cookies:Cookies) -> Template {  // <- request handler
    Template::render("profile",Context::row(&connection, cookies))
}*/
#[get("/creater/account/<account>", rank = 2)]              // <- route attribute
fn user(connection: db::Connection, mut cookies:Cookies, account:String) -> Template {  // <- request handler
    match cookies.get_private("account") {
        Some(c) => if c.value() == account.as_str(){
            //そのユーザー自身がユーザー自身のページに入ったとき
            return Template::render("profile",Context::row(&connection, cookies))
        }else {
            return Template::render("profile",Context::account_row(&connection, account, cookies))
        }
        None => return Template::render("profile",Context::account_row(&connection, account, cookies))

    }
}
#[get("/creater/account/news")]              // <- route attribute
fn news(connection: db::Connection, cookies:Cookies) -> Template {  // <- request handler
    Template::render("news_creater",Context::row(&connection, cookies))
}

//getメソッド群終わり



use rocket::http::Cookies;



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






#[derive(Debug,Serialize)]
struct Context{
    post_img: Vec<news_posts::PostImg>,
    profile: Vec<creater_setting::Profile>,
    user_lisence: bool
    //user_lisenceがfalse == そのページの所有者とそのユーザーは一致しない。
}


use rocket::response::Flash;


extern crate rocket_static_fs;


#[get("/<path..>", rank = 7)]
//creater/hogehogeにstaticディレクトリを適用する
fn files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/post_image").join(path)).ok()
}

impl Context{
    fn row_image(connection: &db::Connection, mut cookies:Cookies) -> Context{
        if let Some(ac) = cookies.get("adult_check"){
            //adult => 1
            //not adult => 0
            println!("adult");
            Context{
                post_img: news_posts::read_gallary(&connection, ac.value().to_string()),
                profile:creater_setting::read_profiles_all(&connection),
                user_lisence: false,
            }
        }else {
            println!("else");
            Context{
                post_img: news_posts::read_gallary(&connection, "0".to_string()),
                profile:creater_setting::read_profiles_all(&connection),
                user_lisence: false,
            }
        }
    }
/*    fn row_news(connection: &db::Connection, mut cookies:Cookies) -> Context{
        if let Some(ac) = cookies.get("adult_check"){
            //adult => 1
            //not adult => 0
            println!("adult");
            Context{
                post_img: news_posts::read_gallary(&connection, ac.value().to_string()),
                profile:creater_setting::read_profiles_all(&connection),
                user_lisence: false,
            }
        }else {
            println!("else");
            Context{
                post_img: news_posts::read_gallary(&connection, "0".to_string()),
                profile:creater_setting::read_profiles_all(&connection),
                user_lisence: false,
            }
        }
    }*/
    fn row(connection: &db::Connection,mut cookies:Cookies) -> Context{
        //この関数でstructのメンバに値が渡される。＝＞"cookieのaccount"由来のものしか出さない。
        //つまり、account="hoge"で画像をpostしても、account="hage"のページでは表示されない。
        let aaa = cookies.get_private("account");
        if let Some(ac) = cookies.get("adult_check"){
            Context{
                post_img: news_posts::read_post_img(connection, aaa.clone(), ac.value().to_string()),
                profile:creater_setting::read_profile(&connection, aaa.clone()),
                user_lisence: true,
            }
        }else {
            Context{
                post_img: news_posts::read_post_img(connection, aaa.clone(), "0".to_string()),
                profile:creater_setting::read_profile(&connection, aaa.clone()),
                user_lisence: true,
            }
        }
    }
    fn account_row(connection: &db::Connection, account:String,cookies:Cookies) -> Context{
        //この関数でstructのメンバに値が渡される。＝＞"cookieのaccount"由来のものしか出さない。
        //つまり、account="hoge"で画像をpostしても、account="hage"のページでは表示されない。
        if let Some(ac) = cookies.get("adult_check"){
            Context{
                post_img: news_posts::read_post_img_normal(connection, account.clone(),ac.value().to_string() ),
                profile:creater_setting::read_profile_normal(&connection, account.clone()),
                user_lisence: false,
            }
        }else {
            Context{
                post_img: news_posts::read_post_img_normal(connection, account.clone(),"0".to_string()),
                profile:creater_setting::read_profile_normal(&connection, account.clone()),
                user_lisence: false,
            }
        }
    }
}


#[get("/creater/account/new")]
fn user_setting(connection: db::Connection, cookies:Cookies) -> Template {
    Template::render("creater_setting", Context::row(&connection, cookies))
}


#[get("/images")]              // <- route attribute
fn images(connection: db::Connection,mut cookie:Cookies) -> Template {  // <- request handler
    if let Some(_) = cookie.get_private("account"){
        let mut context = Context::row_image(&connection, cookie);
        let context = Context{user_lisence:true, .. context};

        return Template::render("gallary", &context)
    }else {
        return Template::render("gallary", Context::row_image(&connection, cookie));
    }
}
#[delete("/creater/account/delete/<id>")]
fn delete(mut cookie:Cookies, id: i32, conn: db::Connection) -> Result<Flash<Redirect>, ()> {
    if news_posts::posts_delete(id, &conn) {
        Ok(Flash::success(Redirect::to(format!("/creater/account/{}",cookie.get_private("account").unwrap().value()).as_str()), ""))
    } else {
        Err(())
    }
}

/*#[get("/creater/<account>")]              // <- route attribute
fn creater(account: User, connection: db::Connection) -> Template {  // <- request handler
    Template::render("creater_1", Context::row(&connection))
}*/

#[get("/creater")]              // <- route attribute
fn creater(cookie:Cookies,connection: db::Connection) -> Template {  // <- request handler
    Template::render("creaters", ProfileContext::row(cookie, &connection))
}

//profile郡
#[derive(Debug,Serialize)]
struct ProfileContext{
    profile: Vec<creater_setting::Profile>,
    user_lisence:bool
}

impl ProfileContext{
    fn row(mut cookie:Cookies,connection: &db::Connection) -> ProfileContext{
        if let Some(_) = cookie.get_private("account"){
            let profile = ProfileContext{
                profile:creater_setting::read_profiles_all(&connection),
                user_lisence: true
            };
            return profile
        }else {
            let profile = ProfileContext{
                profile:creater_setting::read_profiles_all(&connection),
                user_lisence: false
            };
            return profile
        }
    }
}


fn main() {
    rocket::ignite()
        .mount("/", routes![
home,about_me,signup,login,signup_post,multipart_user_setting,
all,creater_static,files,creater,user_setting,profile_static,user,news,images,signin_post,signout_post,delete,help
])
        .mount("/creater/account/post/", routes![multipart_upload])
        .manage(db::connect())
        .attach(Template::fairing())
        .catch(catchers![not_found])
        .launch();
}