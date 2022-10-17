use actix::Message;

pub type GetStateResult = ();

/// Empty message
#[derive(Message)]
#[rtype(result = "GetStateResult")]
pub struct GetStateMessage {}
