
use subxt::{
    ClientBuilder, DefaultConfig,
    PolkadotExtrinsicParams, extrinsic::ExtrinsicParams
};

use crate::Launcher;

use thiserror::Error;

use self::polkadot::runtime_types::polkadot_parachain::primitives::Id;
#[subxt::subxt(runtime_metadata_path = "metadata2.scale")]
pub mod polkadot {}

#[derive(Debug, Error)]
pub enum RegistrarError {
    #[error("something went wrong while registrating parachain")]
    RegistrationFailed
}

pub async fn register_parachain(launcher: &Launcher) -> Result<(), RegistrarError> {
    let api = ClientBuilder::new()
        .set_url("ws://localhost:443")
        .build().await.unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    if let Some(next_id) = reserve_slot(&api) {
        return parainitialize(&api, next_id)
    }

    Err(RegistrarError::RegistrationFailed)
}

fn reserve_slot<T, X>(runtime: &polkadot::RuntimeApi<T, X>) -> Option<Id> 
    where T: subxt::Config, X: ExtrinsicParams<T>
{
    async {

        let genesis = runtime.client.genesis();

        let next_free_id = runtime
            .storage()
            .registrar()
            .next_free_para_id(Some(*genesis)).await.unwrap();
        //NOTE & TODO: RESERVE IS MISSING FROM METADATA
        //let reserved = runtime.storage().registrar().reserve(next_free_id).await.unwrap();

        Some(next_free_id)

    };
    None
}

fn parainitialize<T, X>(runtime: &polkadot::RuntimeApi<T, X>, chain_id: Id) -> Result<(), RegistrarError> 
    where T: subxt::Config, X: ExtrinsicParams<T> {
    
    let extrinsic = subxt::SubmittableExtrinsic::new(&runtime.client, None);
    //let initialize = runtime.client.rpc().submit_extrinsic( )

    Ok(())
}