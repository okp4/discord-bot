//! Holds request commands functions


use crate::cli::application::APP;
use crate::cosmos::client::account::Account;
use crate::cosmos::client::error::Error as ChainError;
use crate::cosmos::client::Client as GRPCClient;
use crate::cosmos::faucet::messages::request_funds::RequestFunds;
use crate::discord_server::Actors;
use crate::discord_server::cmd::CommandExecutable;
use crate::discord_server::error::{Error, ErrorKind};
use abscissa_core::Application;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{BaseAccount, QueryAccountRequest};
use cosmrs::proto::prost;
use cosmrs::tx::{Body, Fee, SignDoc, SignerInfo};
use cosmrs::Error as CosmosError;
use cosmrs::AccountId;
use serenity::async_trait;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use tonic::transport::Channel;

/// A command to ask chain to receive token
pub struct RequestCmd {
    /// Wallet address which will receive token
    pub(crate) address: String,
    pub actors: Actors,
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

        self.actors.faucet.do_send(RequestFunds{
            address: self.address.parse().map_err(|_| {
                Error::from(ErrorKind::Chain(ChainError::Cosmos(
                    CosmosError::AccountId {
                        id: self.address.to_string(),
                    },
                )))
            })?
        });

        Ok(())

        /*
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
            .map_err(Error::from)
        */
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
