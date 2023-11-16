use actix::Message;
use cosmos_sdk_proto::cosmos::staking::v1beta1::Validator;

pub type UpdateStateResult = ();

#[derive(Message)]
#[rtype(result = "UpdateStateResult")]
pub struct UpdateStateMessage {
    pub validators: Vec<Validator>,
}
