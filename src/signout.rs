use rocket::response::Flash;

use rocket::response::Redirect;


use rocket::http::Cookies;

#[get("/creater/profile/signout")]
//signup.html.teraからわたされたSignup structをDBへ取り込み、なおかつaccountの値をcookieに追加して、creater_setting.html.teraへリダイレクトする
fn signout_post(mut cookies: Cookies) -> Flash<Redirect>{
    let account = cookies.get_private("account").unwrap();
    cookies.remove_private(account);
    Flash::success(Redirect::to("/"), "")
}
