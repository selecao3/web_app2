use multipart::server::Multipart;


use multipart::server::save::Entries;

use multipart::server::save::SaveResult::*;

use rocket::Data;
use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;

use std::io::{self, Cursor, Write};
use rocket::response::Redirect;
use std::env;

use regex::Regex;


mod schema {
    table! {
    post_img (id) {
        id -> Nullable<Int4>,
        account -> Varchar,
        img_url_1 -> Text,
        regulation -> Bool,

    }
}
}

use self::schema::post_img;
use self::schema::post_img::dsl::{post_img as all_post_img, regulation as post_img_regulation};

#[derive(Serialize, Queryable, Debug,Clone,Insertable)]
#[table_name = "post_img"]
pub struct PostImg {
    id: Option<i32>,
    account: String,
    img_url_1: String,
    regulation: bool
}

#[derive(FromForm)]
struct PostImgForm{
    img_url_1: String,
    /*    regulation: bool,*/
}



#[post("/unko", data = "<data>")]
// signature requires the request to have a `Content-Type`
fn multipart_upload(cont_type: &ContentType, data: Data, conn:Connection) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>> {
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
        || Custom(
            Status::BadRequest,
            "`Content-Type: multipart/form-data` boundary param not provided".into()
        )
    )?;
    //boundaryの取得

    match process_upload(boundary, data,conn) {
        Ok(resp) => Ok(Stream::from(Cursor::new(resp))),
        Err(err) => Err(Custom(Status::InternalServerError, err.to_string()))
    }
}

fn process_upload(boundary: &str, data: Data, conn:Connection) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();
    println!("process_upload関数");

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().with_dir("static/post_image") {
        //全てのフィールドを一旦保存する
        Full(entries) => process_entries(entries, &mut out, conn)?,
        //成功,entriesにはフィールドが全て詰まっている
        Partial(partial, reason) => {
            //途中で失敗した。
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out, conn)?
        },
        Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to

use multipart::server::FieldHeaders;
use multipart::server::save::SavedField;
use multipart::server::save::SaveDir::Perm;
use multipart::server::save::SavedData;
use std::path::PathBuf;
use db;
use db::Connection;
use std::fs::rename;
use std::mem::replace;


fn process_entries(entries: Entries, mut out: &mut Vec<u8>, conn:Connection) -> io::Result<()> {
    {

        /*        println!("======¥n{:?}¥n========",entries.fields.get(&"file".to_string()).unwrap().get(0));*/
        let aaa = &entries.fields.get(&"file".to_string()).unwrap().get(0).unwrap().data;
        if let SavedData::File(bbb,ccc) = aaa{
            println!("{:?}", bbb);
             let mut s = bbb.to_str().unwrap().to_string();
            s.push_str(".png");
            rename(bbb.to_str().unwrap(),s.trim() ).unwrap();
            //file名を*.pngに変更している.


/*            let re = Regex::new(r"[static/]").unwrap();
            let result = re.replace_all(s.trim(), "");*/


            let t = PostImgForm{
                img_url_1: s.trim_left_matches('/').to_string()
            };
            insert(t,&conn);

        }

        /*        let hoge = entries.save_dir;
                let hage = Perm(hoge.into_path());*/


        /*        println!("{:?}", aaa.get(0).unwrap().data::File);*/

    }

    writeln!(out, "Entries processed")
}

use diesel::pg::PgConnection;
use diesel;
use diesel::prelude::*;
fn insert(postimgform:PostImgForm, conn: &PgConnection) -> bool{
    let t = PostImg{
        id: None,
        account: "root".to_string(),
        img_url_1: postimgform.img_url_1,
        //保存したimg_urlをどうにかしてPost structへ・・・
        regulation: false
    };
    diesel::insert_into(post_img::table).values(&t).execute(conn).is_ok()
}
pub fn read_post_img(connection: &PgConnection) -> Vec<PostImg> {
    //postsテーブルからデータを読み取る。
    all_post_img
        .order(post_img::id.desc())
        .load::<PostImg>(connection)
        .expect("error")
}