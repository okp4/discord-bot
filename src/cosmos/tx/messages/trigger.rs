use actix::Message;

pub type TriggerTxResult = ();

#[derive(Message)]
#[rtype(result = "TriggerTxResult")]
pub struct TriggerTx {
    /// The memo field content of the transaction.
    pub memo: String,

    /// gas limit of the transaction.
    pub gasLimit: u64
}
