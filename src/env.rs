use dotenv;
use std::env::{self, VarError};
use std::path::PathBuf;

use crate::env::dotenv::Error;
use config::Config;
use directories;

#[derive(Debug)]
pub struct Envs {
    pub incoming: PathBuf,
    pub save_path_a: PathBuf,
    pub save_path_d: PathBuf,
    pub save_path_g: PathBuf,
}

impl Envs {
    pub fn load_envs() {
        dotenv::dotenv().ok();
    }

    //        unsafe { env::set_var("INCOMING_PATH", path) }
    pub fn read_incoming_path() -> Result<String, VarError> {
        env::var("INCOMING_PATH")
    }
}
