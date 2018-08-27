use multipart::server::Multipart;


use multipart::server::save::Entries;

use multipart::server::save::SaveResult::*;

use rocket::Data;
use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;

use std::io::{self, Write};
use rocket::response::Redirect;


use chrono::Local;
use chrono_tz::Tz;

pub const JAPAN: Tz = Tz::Japan;


pub use schema::profile;
pub use schema::profile::dsl::profile as all_profile;

#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "profile"]
pub struct Profile{
    id: Option<i32>,
    name: String,
    account: String,
    profile_text: String,
    profile_img : String,
    created_day:String

}

#[derive(FromForm)]
struct ProfileForm{
    name: String,
    profile_text: String,
    profile_img: String,
    /*    regulation: bool,*/
}

use rocket::http::Cookies;


#[post("/creater/user/setting", data = "<data>")]
// signature requires the request to have a `Content-Type`
fn multipart_user_setting(cont_type: &ContentType, data: Data, conn:Connection,mut cookies:Cookies) -> Result<Redirect, Custom<String>> {
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
        || Custom(
            Status::BadRequest,
            "`Content-Type: multipart/form-data` boundary param not provided".into()
        )
    )?;
    //boundaryの取得
    let cookie = cookies.get_private("account");

    match process_upload(boundary, data,conn,cookies) {
        Ok(_) => Ok(Redirect::to(format!("/creater/account/{}",cookie.unwrap().value()).as_str())),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
    }
}

fn process_upload(boundary: &str, data: Data, conn:Connection,cookies:Cookies) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    println!("process_upload関数");

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().size_limit(None).with_dir("static/profile_imgs"){
        //全てのフィールドを一旦保存する
        Full(entries) => process_entries(entries, &mut out, conn, cookies)?,
        //成功,entriesにはフィールドが全て詰まっている
        Partial(partial, reason) => {
            //途中で失敗した。
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out, conn,cookies)?
        },
        Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to

use multipart::server::save::SavedData;
use db::Connection;
use std::fs::rename;


fn process_entries(entries: Entries, out: &mut Vec<u8>, conn:Connection,cookies:Cookies) -> io::Result<()> {
    {

        /*        println!("======¥n{:?}¥n========",entries.fields.get(&"file".to_string()).unwrap().get(0));*/
        let mut tmp:Vec<String> = Vec::new();

        let name= &entries.fields.get(&"name".to_string()).unwrap().get(0).unwrap().data;
        let profile_text = &entries.fields.get(&"profile_text".to_string()).unwrap().get(0).unwrap().data;
        let profile_img= &entries.fields.get(&"profile_img".to_string()).unwrap().get(0).unwrap().data;



        if let SavedData::File(bbb,_) = profile_img{
            println!("{:?}", bbb);
            let mut s = bbb.to_str().unwrap().to_string();
            s.push_str(".png");
            rename(bbb.to_str().unwrap(),s.trim() ).unwrap();
            println!("{}",s.trim_left_matches("static/").to_string());
            //file名を*.pngに変更している.
            let file_path = s.trim_left_matches("static/").to_string();

            tmp.push(file_path);
        }
        if let SavedData::Text(name_string) = name{
            println!("{}",name_string);
            tmp.push(name_string.to_string());
        }
        if let SavedData::Text(profile_string) = profile_text{
            println!("{}",profile_string);
            tmp.push(profile_string.to_string());
        }
        let t = ProfileForm{
            profile_img:tmp[0].clone(),
            name:tmp[1].clone(),
            profile_text:tmp[2].clone(),
        };
        update(t, &conn,cookies);
    }

    writeln!(out)
}

use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;

//signup時のみ呼ばれる関数。初期値をセットする。
pub fn insert(conn: &PgConnection,mut cookies: Cookies) -> bool{
    println!("insertメソッド");

    let t = Profile{
        id: None,
        account:cookies.get_private("account").unwrap().value().to_string(),
        name:"".to_string(),
        profile_text: "".to_string(),
        profile_img: "".to_string(),
        //保存したimg_urlをどうにかしてPost structへ・・・
        created_day: Local::now().with_timezone(&JAPAN).to_rfc3339()
    };
    diesel::insert_into(profile::table).values(&t).execute(conn).is_ok()
}

fn update(profile:ProfileForm, conn: &PgConnection,mut cookies: Cookies) -> bool{
    println!("updateメソッド");

    let t = Profile{
        id: None,
        account:cookies.get_private("account").unwrap().value().to_string(),
        name:profile.name,
        profile_text: profile.profile_text,
        profile_img: profile.profile_img,
        //保存したimg_urlをどうにかしてPost structへ・・・
        created_day: Local::now().with_timezone(&JAPAN).to_rfc3339()

    };
    diesel::update(all_profile
        .filter(profile::account.eq(cookies.get_private("account").unwrap().value())))
        .set((
            profile::name.eq(&t.name),
            profile::profile_text.eq(&t.profile_text),
            profile::profile_img.eq(&t.profile_img),
            profile::created_day.eq(&t.created_day),
        )
        )
        .execute(conn)
        .is_ok()
}




use rocket::http::Cookie;

pub fn read_profile(connection: &PgConnection, cookies: Option<Cookie>) -> Vec<Profile> {
    //postsテーブルからデータを読み取る。

    all_profile
        //accountが◯◯のものを取り出す
        /*        .filter(profile::account.eq(cookies.map(|c| c.value()).unwrap()))*/
        .filter(profile::account.eq(cookies.unwrap().value()))
        .order(profile::id.desc())
        .load::<Profile>(connection)
        .expect("error")
}
pub fn read_profile_normal(connection: &PgConnection, account: String) -> Vec<Profile> {
    //postsテーブルからデータを読み取る。
    all_profile
        //accountが◯◯のものを取り出す
        .filter(profile::account.eq(account.as_str()))
        .order(profile::id.desc())
        .load::<Profile>(connection)
        .expect("error")
}

pub fn read_profiles_all(connection: &PgConnection) -> Vec<Profile> {
    //postsテーブルから全てデータを読み取る。
    all_profile
        .order(profile::id.desc())
        .load::<Profile>(connection)
        .expect("error")
}
