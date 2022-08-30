use cosmrs::bank::MsgSend;
use actix::Message;
use cosmrs::tx::Fee;

pub type MakeTxResult = ();

#[derive(Message)]
#[rtype(result = "MakeTxResult")]
pub struct MakeTx {
    /// Contains the messages to embed in the transaction.
    pub msgs: Vec<MsgSend>,

    /// The memo field content of the transaction.
    pub memo: String,

    /// Fees of the transaction determined from amount and gas.
    pub fee: Fee
}
