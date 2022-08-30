use cosmrs::auth::BaseAccount;
use cosmrs::bank::MsgSend;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::{Body, Fee, SignDoc, SignerInfo};
use crate::cosmos::tx::error::Error;

pub struct TxHandler {
    pub chainId: String,
    pub msgs: Vec<MsgSend>,
}

impl TxHandler {

    fn sign_tx(self,
        body: &Body,
        account: BaseAccount,
        signing_key: SigningKey,
        fee: Fee
    ) -> Result<Vec<u8>, Error> {

        let public_key = signing_key.public_key();
        let signer_info = SignerInfo::single_direct(Some(public_key), account.sequence);

        let auth_info = signer_info.auth_info(fee);

        let sign_doc = SignDoc::new(
            body,
            &auth_info,
            &self.chainId.parse().map_err(|_| Error::IncorrectChain)?,
            account.account_number,
        )?;

        let tx_signed = sign_doc.sign(&signing_key)?;

        tx_signed.to_bytes().map_err(Error::from)
    }
}
