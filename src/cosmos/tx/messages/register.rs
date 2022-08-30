use cosmrs::bank::MsgSend;
use actix::Message;

pub type RegisterTxResult = ();

#[derive(Message)]
#[rtype(result = "RegisterTxResult")]
pub struct RegisterTx {
    /// Contains the messages to embed in the transaction.
    pub msg: MsgSend,
}
