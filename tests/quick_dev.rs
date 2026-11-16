#![allow(unused)]

use anyhow::{Ok, Result};

#[tokio::test]
async fn quick_dev() -> Result<(),anyhow::Error> {
  let hc = httpc_test::new_client("http://localhost:8080")?;

  hc.do_get("/hello?name=miracle").await?.print().await?;
  Ok(())
}