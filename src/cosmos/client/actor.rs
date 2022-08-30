use actix::{Actor, Context};
use tonic::transport::Channel;

use super::Client;

impl Actor for Client<Channel> {
    type Context = Context<Self>;
}
