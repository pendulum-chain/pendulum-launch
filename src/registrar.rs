use std::fmt;

use subxt::{
    ClientBuilder, DefaultConfig,
    PolkadotExtrinsicParams
};

use crate::Launcher;

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod polkadot {}

#[derive(Debug)]
struct RegistrarError {
    message: String,
}

impl fmt::Display for RegistrarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub async fn register_parachain(launcher: &Launcher) -> Result<(), RegistrarError> {
    let api = ClientBuilder::new()
        .set_url("ws://localhost:443")
        .build()
        .await.unwrap()
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, PolkadotExtrinsicParams<DefaultConfig>>>();

    if let Some(next_id) = reserve_slot(&api) {
        return parainitialize(&api, next_id)
    }

    Err(RegistrarError {
        message: "failed to register parachain.".to_string()
    })
}

fn reserve_slot<T, X>(runtime: &polkadot::RuntimeApi<T, X>) -> Option<u32> 
    where T: subxt::Config
{
    runtime.client.registrar.reserve()
}

fn parainitialize<T, X>(runtime: &polkadot::RuntimeApi<T, X>, chain_id: u32) -> Result<(), RegistrarError> 
    where T: subxt::Config {
    runtime.client.paras_sudo_wrapper
        .sudo_schedule_parainitialize(chain_id, b"genesis")
}