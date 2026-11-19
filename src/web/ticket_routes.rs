use axum::routing::{post,get,delete};
use axum::{Json, Router};
use axum::extract::{Path, State};
use serde_json::{Value, json};

use crate::context::Ctx;
use crate::error::{Error, Result};
use crate::model::{CreateTicketDto, ModelController, Ticket};


// route

pub fn ticket_routes(mc:ModelController) -> Router {
  Router::new()
  .route("/tickets", post(create_ticket).get(get_tickets))
  .route("/tickets/:id", delete(delete_ticket)).with_state(mc)
}


// rest handlers
pub async fn create_ticket(
    State(mc): State<ModelController>,
    ctx:Ctx,
    Json(ticket_dto): Json<CreateTicketDto>,
) -> Result<Json<Ticket>> {
    println!("->> {:<15} - create_ticket fn REST", "HANDLER");
    let ticket = mc.create_ticket(ctx,ticket_dto).await?;

    Ok(Json(ticket))
}

pub async fn get_tickets(State(mc): State<ModelController>,ctx:Ctx) -> Result<Json<Value>> {
    println!("->> {:<15} - get_tickets fn REST", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(json!({
      "tickets":tickets
    })))
}

pub async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx:Ctx,
    Path(ticket_id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket fn REST", "HANDLER");

    let ticket = mc.delete_ticket(ctx,ticket_id).await?;

    Ok(Json(ticket))
}
