use crate::cosmos::faucet::Faucet;
use actix::{Actor, Context};

impl Actor for Faucet {
    type Context = Context<Self>;
}
