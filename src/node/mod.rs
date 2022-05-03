use crate::Result;
use std::process;

mod base;
mod collator;
mod validator;

pub use base::BaseNode;
pub use collator::{Collator, CollatorRelay};
pub use validator::Validator;

pub trait Node {
    fn name(&self) -> &str;
    fn args(&self) -> Result<Vec<String>>;
    fn ports(&self) -> Vec<Option<u16>>;
    fn specs(&self) -> Result<Vec<String>>;
    fn docker_file(&self) -> Result<String>;
}

pub trait AsCommand {
    fn as_command_internal(&self) -> Result<process::Command>;
    // TODO: move docker_volume flag into cli
    fn as_command_external(&self, docker_volume: bool) -> Result<String>;
}
