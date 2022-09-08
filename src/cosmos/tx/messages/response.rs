use serenity::model::user::User;
use crate::cosmos::tx::error::Error;
use actix::Message;

pub type TxResponseResult = ();

#[derive(Message)]
#[rtype(result = "TxResponseResult")]
pub struct TxResponse {
    pub response: Result<cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse, Error>,
    pub subscribers: Vec<User>,
}
