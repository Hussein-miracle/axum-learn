
use crate::{error::{Error,Result}, web};
use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookie, Cookies, cookie};


pub fn routes() -> Router {
  Router::new().route("/api/login", post(api_login))
}

#[derive(Debug,Deserialize)]
struct LoginPayload {
  username:String,
  password:String
}


async fn api_login(cookies:Cookies,payload:Json<LoginPayload>) -> Result<Json<Value>> {
  println!("->> {:<12} - api_login - {payload:?}","HANDLER");

  // TODO : implement real db/auth logic
  if payload.username != "demo1" || payload.password != "welcome" {
    return Err(Error::LoginFailure);
  }

  // TODO: impl real auth cookie

  cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));


  // create success body
  let body= Json(json!({
    "result":{
      "success":true
    }
  }));

  Ok(body)
}