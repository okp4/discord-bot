use actix::{Actor, Context};
use crate::cosmos::tx::tx::TxHandler;

impl Actor for TxHandler {
    type Context = Context<Self>;
}
