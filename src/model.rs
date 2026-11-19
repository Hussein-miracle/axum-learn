//! simplistic model layer
//! (with mock-store layer)

use crate::{context::Ctx, error::{Error, Result}};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// start_region : ticket types
#[derive(Debug, Clone, Serialize)]
pub struct Ticket {
   pub  id: u64,
   pub  cid:u64, // ticker creator
   pub  title: String,
}

#[derive(Deserialize)]
pub struct CreateTicketDto {
    pub title: String,
}
// end_region : ticket types

// start_region : model controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::default(),
        })
    }
}

// CRUD
impl ModelController {
    pub async fn create_ticket(&self,ctx:Ctx, ticket_dto: CreateTicketDto) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = (store.len() + 1) as u64;

        let ticket = Ticket {
            id,
            cid:ctx.user_id(),
            title: ticket_dto.title,
        };
        store.push(Some(ticket.clone()));

        Ok(ticket)
    }

    pub async fn list_tickets(&self,_ctx:Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store
            .iter()
            .filter_map(|t| t.clone())
            .collect::<Vec<Ticket>>();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self,_ctx:Ctx, ticket_id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let ticket_to_delete = store
            .get_mut((ticket_id - 1) as usize)
            .and_then(|t| t.take());

        ticket_to_delete.ok_or(Error::TicketNotFound { id: ticket_id })
    }
}
// end_region : model controller
