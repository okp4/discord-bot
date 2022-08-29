//! Holds request commands functions

use crate::application::APP;
use crate::grpc::account::Account;
use crate::grpc::client::Client as GRPCClient;
use crate::grpc::error::Error as ChainError;
use crate::discord::cmd::CommandExecutable;
use crate::discord::error::{Error, ErrorKind};
use abscissa_core::Application;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmos_sdk_proto::cosmos::tx::v1beta1::BroadcastTxRequest;
use cosmrs::bank::MsgSend;
use cosmrs::proto::prost;
use cosmrs::tx::{Body, Fee, Msg, SignDoc, SignerInfo};
use cosmrs::Error as CosmosError;
use cosmrs::{AccountId, Coin};
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use tonic::transport::Channel;
use tonic::Status;

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
        grpc_client: &GRPCClient<Channel>,
    ) -> Result<(), Error> {
        let config = &APP.config();

        let sender = Account::new(config.faucet.mnemonic.clone(), &config.chain.prefix)?;

        let amount = Coin {
            amount: config.faucet.amount_send as u128,
            denom: config.chain.denom.parse().unwrap(),
        };

        let msg_send = MsgSend {
            from_address: sender.address.clone(),
            to_address: self.address.parse().map_err(|_| {
                Error::from(ErrorKind::Chain(ChainError::Cosmos(
                    CosmosError::AccountId {
                        id: self.address.to_string(),
                    },
                )))
            })?,
            amount: vec![amount.clone()],
        };

        let gas = config.faucet.gas_limit;
        let timeout_height = 0u16;
        let memo = &config.faucet.memo;

        let tx_body = Body::new(vec![msg_send.to_any().unwrap()], memo, timeout_height);

        let tx_signed = sign_tx(
            grpc_client,
            &tx_body,
            sender,
            Fee::from_amount_and_gas(amount, gas),
        )
        .await?;

        let request = tonic::Request::new(BroadcastTxRequest {
            tx_bytes: tx_signed,
            mode: 2,
        });

        let tx_response = grpc_client
            .clone()
            .tx()
            .broadcast_tx(request)
            .await
            .and_then(|resp| {
                resp.get_ref()
                    .tx_response
                    .clone()
                    .ok_or_else(|| Status::not_found("No transaction response"))
            })?;

        let content = format!(
            "💵 You will receive {}{}.
            \t- 🤝 Transaction hash: {}
            \t- ⚙️ Result code : {}
            \t- 📝 Raw log : {}
            \t- ⛽️ Gas used: {}",
            config.faucet.amount_send,
            config.chain.denom,
            tx_response.txhash,
            tx_response.code,
            tx_response.raw_log,
            tx_response.gas_used
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
    M::decode(&msg.value[..]).ok()
}

async fn get_account(client: &GRPCClient<Channel>, addr: AccountId) -> Result<BaseAccount, Error> {
    let request = tonic::Request::new(QueryAccountRequest {
        address: addr.to_string(),
    });

    let response = client
        .clone()
        .auth()
        .account(request)
        .await
        .map_err(Error::from)?;

    let account_response = response
        .get_ref()
        .account
        .as_ref()
        .ok_or(CosmosError::AccountId {
            id: addr.to_string(),
        })
        .map_err(Error::from)?;
    Ok(
        unpack_from_any(account_response).ok_or(CosmosError::AccountId {
            id: addr.to_string(),
        })?,
    )
}

async fn sign_tx(
    client: &GRPCClient<Channel>,
    body: &Body,
    sender: Account,
    fee: Fee,
) -> Result<Vec<u8>, ChainError> {
    let config = APP.config();

    let account = get_account(client, sender.address.clone())
        .await
        .map_err(|e| ChainError::Connection(e.to_string()))?;

    let public_key = sender.signing_key()?.public_key();
    let signer_info = SignerInfo::single_direct(Some(public_key), account.sequence);

    let auth_info = signer_info.auth_info(fee);

    let sign_doc = SignDoc::new(
        body,
        &auth_info,
        &config.chain.chain_id.parse()?,
        account.account_number,
    )?;

    let tx_signed = sign_doc.sign(&sender.signing_key()?)?;

    tx_signed.to_bytes().map_err(ChainError::from)
}
