use crate::cosmos::tx::TxHandler;
use actix::{Actor, Context};

impl Actor for TxHandler {
    type Context = Context<Self>;
}
