use crate::cosmos::tx::tx::TxHandler;
use actix::{Actor, Context};

impl Actor for TxHandler {
    type Context = Context<Self>;
}
