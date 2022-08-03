use crate::application::APP;
use crate::chain::faucet::FaucetClient;
use crate::discord::cmd::CommandExecutable;
use crate::discord::error::Error;
use abscissa_core::Application;
use cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmos_sdk_proto::cosmos::tx::v1beta1::service_client::ServiceClient;
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use cosmrs::bank::MsgSend;
use cosmrs::proto::prost;
use cosmrs::tendermint::chain::Id;
use cosmrs::tx::{Fee, Msg, SignDoc, SignerInfo};
use cosmrs::Error as CosmosError;
use cosmrs::{tx, AccountId, Coin};
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};

/// A command to ask chain to receive token
pub struct RequestCmd {
    /// Wallet address which will receive token
    pub(crate) address: String,
}

/// Execute the "ping" command.
#[async_trait]
impl CommandExecutable for RequestCmd {
    async fn execute(
        &self,
        ctx: &Context,
        _: &Interaction,
        command: &ApplicationCommandInteraction,
    ) -> Result<(), Error> {
        let config = &APP.config();

        let faucet = FaucetClient::new(&config.faucet.mnemonic)?;

        let account = get_account(faucet.address.clone()).await.unwrap();

        let amount = Coin {
            amount: config.faucet.amount_send as u128,
            denom: config.chain.denom.parse().unwrap(),
        };

        let msg_send = MsgSend {
            from_address: faucet.address.clone(),
            to_address: self.address.parse().unwrap(),
            amount: vec![amount.clone()],
        };

        let chain_id: Id = config.chain.chain_id.parse().unwrap();
        let account_number = account.account_number;
        let sequence_number = account.sequence;
        let gas = config.faucet.gas_limit;
        let timeout_height = 0u16;
        let memo = &config.faucet.memo;

        let tx_body = tx::Body::new(vec![msg_send.to_any().unwrap()], memo, timeout_height);

        let signer_info =
            SignerInfo::single_direct(Some(faucet.signing_key.public_key()), sequence_number);

        let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(amount, gas));

        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, account_number).unwrap();

        let tx_signed = sign_doc.sign(&faucet.signing_key).unwrap();

        let tx_bytes = tx_signed.to_bytes().unwrap();

        let mut client = ServiceClient::connect(config.chain.grpc_address.to_string())
            .await
            .unwrap();

        let request = tonic::Request::new(BroadcastTxRequest { tx_bytes, mode: 2 });

        let tx_response = client.broadcast_tx(request).await.unwrap();

        let content = format!(
            "💵 You will receive {}{}.
            \t- 🤝 Transaction hash: {}
            \t- ⚙️ Result code : {}
            \t- 📝 Raw log : {}
            \t- ⛽️ Gas used: {}",
            config.faucet.amount_send,
            config.chain.denom,
            tx_response.get_ref().tx_response.as_ref().unwrap().txhash,
            tx_response.get_ref().tx_response.as_ref().unwrap().code,
            tx_response.get_ref().tx_response.as_ref().unwrap().raw_log,
            tx_response.get_ref().tx_response.as_ref().unwrap().gas_used
        );
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
            .map_err(Error::from)
    }
}

fn unpack_from_any<M>(msg: &prost_types::Any) -> Option<M>
where
    M: prost::Message + Default,
{
    Some(M::decode(&msg.value[..]).ok()?)
}

async fn get_account(
    addr: AccountId,
) -> Result<BaseAccount, Box<dyn std::error::Error + Send + Sync>> {
    let mut client = QueryClient::connect("http://[::1]:9090").await?;
    let request = tonic::Request::new(QueryAccountRequest {
        address: addr.to_string(),
    });
    let response = client.account(request).await?;
    let account_response = response
        .get_ref()
        .account
        .as_ref()
        .ok_or(CosmosError::AccountId {
            id: addr.to_string(),
        })?;
    Ok(
        unpack_from_any(&account_response).ok_or(CosmosError::AccountId {
            id: addr.to_string(),
        })?,
    )
}
