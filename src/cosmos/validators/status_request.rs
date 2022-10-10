//todo should disappear

use std::fmt;

use cosmos_sdk_proto::cosmos::staking::v1beta1::query_client::QueryClient as ValidatorClient;
use cosmos_sdk_proto::cosmos::staking::v1beta1::QueryValidatorsRequest;
use tracing::{info, warn};

use crate::error::Error;

pub struct ValidatorInfo {
    id: String,
    description: String,
    jailed: bool,
    status: i32,
}
impl fmt::Display for ValidatorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Validator {} called {} jailed : {} status :  {}",
            self.id,
            self.description,
            self.jailed.to_string(),
            self.status.to_string()
        )
    }
}
pub async fn check_validator() -> Result<Vec<ValidatorInfo>, Error> {
    info!("check_validator()");
    let mut validator_client =
        ValidatorClient::connect("https://grpc.devnet.staging.okp4.network:443")
            .await
            .unwrap();

    let response = validator_client
        .validators(QueryValidatorsRequest {
            status: "BOND_STATUS_BONDED".to_string(),
            pagination: None,
        })
        .await
        .map_err(|err| warn!("error : {}", err))
        .ok()
        .unwrap();

    let size = response.get_ref().validators.len();
    let mut ret_validators = Vec::with_capacity(size);
    for validator in &response.get_ref().validators {
        ret_validators.push(ValidatorInfo {
            // id: validator.tokens.clone(),
            id: validator.description.as_ref().unwrap().identity.clone(),
            description: validator.description.as_ref().unwrap().moniker.to_string(),
            jailed: validator.jailed,
            status: validator.status,
        });
    }
    Ok(ret_validators)
}
