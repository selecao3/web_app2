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

/*use comrak::{markdown_to_html, ComrakOptions};*/



use schema::post_img;
use schema::post_img::dsl::post_img as all_post_img;

#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "post_img"]
pub struct PostImg {
    id: Option<i32>,
    account: String,
    name: String,
    title: String,
    body: String,
    img_url_1: String,
    img_url_2: String,
    img_url_3: String,
    img_url_4: String,
    adult_check: String,
    created_day:String
}

#[derive(FromForm)]
struct PostImgForm{
    title: String,
    body: String,
    img_url_1: String,
    img_url_2: String,
    img_url_3: String,
    img_url_4: String,
    adult_check: String,
    /*    regulation: bool,*/
}



#[post("/form", data = "<data>")]
// signature requires the request to have a `Content-Type`
fn multipart_upload(cont_type: &ContentType, data: Data, conn:Connection,mut cookies:Cookies) -> Result<Redirect, Custom<String>> {
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
        || Custom(
            Status::BadRequest,
            "`Content-Type: multipart/form-data` boundary param not provided".into()
        )
    )?;
    //boundaryの取得
    let cookie = cookies.get_private("account").clone();

    match process_upload(boundary, data,conn,cookies) {
        Ok(_) => Ok(Redirect::to(format!("/creater/account/{}",cookie.unwrap().value()).as_str())),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
    }
}

fn process_upload(boundary: &str, data: Data, conn:Connection, cookies:Cookies) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().size_limit(None).with_dir("static/post_image"){
        //全てのフィールドを一旦保存する
        Full(entries) => process_entries(entries, &mut out, conn,cookies)?,
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



fn process_entries(entries: Entries, out: &mut Vec<u8>, conn:Connection, cookies:Cookies) -> io::Result<()>{
    {
        let mut tmp:Vec<String> = Vec::new();
        if let Some(title_data) = &entries.fields.get(&"title".to_string()){
            let title = &title_data.get(0).unwrap().data;
            if let SavedData::Text(title_string) = title{
                tmp.push(title_string.to_string());
            }
        }else {
            tmp.push("".to_string());
        }
        if let Some(body_data) = &entries.fields.get(&"body".to_string()){
            let body = &body_data.get(0).unwrap().data;
            if let SavedData::Text(body_string) = body{
                tmp.push(body_string.to_string());
            }
        }else {
            tmp.push("".to_string());
        }

        let adult_check = &entries.fields.get(&"customRadio".to_string()).unwrap().get(0).unwrap().data;
        if let SavedData::Text(adult_check_flag) = adult_check{
            tmp.push(adult_check_flag.to_string());
        }



        for i in 0..4 {
            if let Some(file_data) = &entries.fields.get(&"file[]".to_string()){
                if let Some(file) = file_data.get(i){
                    //fileがuploadされた時
                    if let SavedData::File(bbb,_) = &file.data{
                        println!("{:?}", bbb);
                        let mut s = bbb.to_str().unwrap().to_string();
                        s.push_str(".png");
                        rename(bbb.to_str().unwrap(),s.trim()).unwrap();
                        println!("{}",&s.trim_left_matches("static/").to_string());
                        //file名を*.pngに変更している.
                        let file_path = s.trim_left_matches("static/").to_string();
                        tmp.push(file_path);
                    }else {
                        //そもそもfileがuploadされなかった時。
                        tmp.push("".to_string());
                    }
                }else {
                    //4つのうちいくつかがuploadされなかった時
                    tmp.push("".to_string());
                }
            }
        }


        let t = PostImgForm{
            title:tmp[0].clone(),
            body:tmp[1].clone(),
            adult_check:tmp[2].clone(),
            img_url_1:tmp[3].clone(),
            img_url_2:tmp[4].clone(),
            img_url_3:tmp[5].clone(),
            img_url_4:tmp[6].clone(),
        };
        insert(t,&conn,cookies);


    }
    writeln!(out)
}

use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;

use rocket::http::Cookies;
use rocket::http::Cookie;

use creater_setting;


fn insert(postimgform:PostImgForm, conn: &PgConnection,mut cookies: Cookies) -> bool{
    let cookie = cookies.get_private("account");
    let t = PostImg{
        id: None,
        account: cookie.clone().unwrap().value().to_string().clone(),
        name: creater_setting::all_profile
            .filter(creater_setting::profile::account.eq(cookie.unwrap().value()))
            //profileテーブルのそのユーザーのアカウントが入っている"行"を抽出する
            .select(name)
            //nameの列を抽出
            .first(conn).unwrap(),
        //投稿したユーザーのnameを取ってくる
        title:postimgform.title,
        body: postimgform.body,
        img_url_1: postimgform.img_url_1,
        img_url_2: postimgform.img_url_2,
        img_url_3: postimgform.img_url_3,
        img_url_4: postimgform.img_url_4,
        //保存したimg_urlをどうにかしてPost structへ・・・
        adult_check: postimgform.adult_check,
        created_day: Local::now().with_timezone(&JAPAN).to_rfc3339()
    };
    diesel::insert_into(post_img::table).values(&t).execute(conn).is_ok()
}
use schema::profile::columns::name;


pub fn read_post_img(connection: &PgConnection, cookies:Option<Cookie>,adult_check:String) -> Vec<PostImg> {
    //postsテーブルからデータを読み取る。
    if adult_check == "0".to_string() {
        all_post_img
            .filter(post_img::account.eq(cookies.unwrap().value())
                .and(post_img::adult_check.eq("0")))
            //accountが◯◯のものを取り出す
            .order(post_img::id.desc())
            .load::<PostImg>(connection)
            .expect("error")
    }else {
        all_post_img
            .filter(post_img::account.eq(cookies.unwrap().value()))
            //accountが◯◯のものを取り出す
            .order(post_img::id.desc())
            .load::<PostImg>(connection)
            .expect("error")
    }

}
pub fn read_post_img_normal(connection: &PgConnection, account:String, adult_check:String) -> Vec<PostImg> {
    //postsテーブルからデータを読み取る。

    if adult_check == "0".to_string() {
        all_post_img
            .filter(post_img::account.eq(account.as_str()).and(post_img::adult_check.eq("0")))
            //accountが◯◯のものを取り出す
            .order(post_img::id.desc())
            .load::<PostImg>(connection)
            .expect("error")
    }else {
        all_post_img
            .filter(post_img::account.eq(account.as_str()))
            //accountが◯◯のものを取り出す
            .order(post_img::id.desc())
            .load::<PostImg>(connection)
            .expect("error")
    }
}
pub fn read_gallary(connection: &PgConnection,adult_check:String) -> Vec<PostImg> {
    //postsテーブルからデータを読み取る。
    if adult_check == "0".to_string() {
        all_post_img
            .order(post_img::id.desc())
            .filter(post_img::adult_check.eq("0"))
            .load::<PostImg>(connection)
            .expect("error")
    }else {
        //adultの時
        all_post_img
            .order(post_img::id.desc())
            .load::<PostImg>(connection)
            .expect("error")
    }

}
pub fn posts_delete(id:i32,connection: &PgConnection) -> bool {
    //postsテーブルからデータを読み取る。
    diesel::delete(all_post_img.find(id)).execute(connection).is_ok()
}
