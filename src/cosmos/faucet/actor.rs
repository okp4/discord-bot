use actix::{Actor, Context, Handler};
use cosmrs::bank::MsgSend;
use crate::cosmos::faucet::Faucet;
use crate::cosmos::tx::messages::register::RegisterMsg;

impl Actor for Faucet {
    type Context = Context<Self>;
}
