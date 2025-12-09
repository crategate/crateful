use dotenv;
use std::env;
use std::path::PathBuf;

use config::Config;

#[derive(Debug)]
pub struct Envs {
    pub incoming: PathBuf,
    pub save_path_a: PathBuf,
}

impl Envs {
    pub fn read_from_env() {
        dotenv::dotenv().ok();
    }

    //        unsafe { env::set_var("INCOMING_PATH", path) }
    pub fn read_env_paths() {
        match env::var("INCOMING_PATH") {
            Ok(value) => dbg!("{}", value),
            Err(e) => dbg!("{}", "Fatal Error Reading Env Vars".to_string()),
        };
    }
}
