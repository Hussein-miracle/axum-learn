#![allow(unused)]

use anyhow::{Ok, Result};
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<(),anyhow::Error> {
  let hc = httpc_test::new_client("http://localhost:8080")?;

  hc.do_get("/hello2/mike").await?.print().await?;
  println!("");


  // let req_login_wrong_pwd = hc.do_post(
  //       "/api/login",
  //       json!(
  //           {
  //              "username": "demo1",
  //              "pwd": "not welcome"
  //           }
  //       ),
  //   );
  // req_login_wrong_pwd.await?.print().await?;

  // let req_login_wrong_json = hc.do_post(
  //       "/api/login",
  //       json!(
  //           {
  //              "username": "demo1",
  //              "wrongField": "welcome"
  //           }
  //       ),
  //   );
  // req_login_wrong_json.await?.print().await?;

    let login_request = hc.do_post("/api/login", json!({
    "username":"demo1",
    "password":"welcome"
  }));
  login_request.await?.print().await?;

  //hc.do_get("/hello2/mike").await?.print().await?;
  	let req_create_ticket = hc.do_post(
		"/api/tickets",
		json!(
			{
				"title": "Ticket AAA"
			}
		),
	);

	req_create_ticket.await?.print().await?;
	hc.do_get("/api/tickets").await?.print().await?;
	// hc.do_delete("/api/tickets/1").await?.print().await?;
	// hc.do_get("/api/tickets").await?.print().await?;
  Ok(())
}

