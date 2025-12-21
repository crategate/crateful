use crate::env::dotenv::Error;
use config::Config;
use directories::ProjectDirs;
use dotenv;
use std::env::{self, VarError};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Envs {
    pub incoming: PathBuf,
    pub save_path_a: PathBuf,
    pub save_path_d: PathBuf,
    pub save_path_g: PathBuf,
}
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    dbg!("{}", &filename);
    let file = File::open(filename).expect("well");

    Ok(io::BufReader::new(file).lines())
}
impl Envs {
    pub fn load_envs() {
        dotenv::dotenv().ok();
    }

    pub fn try_config_load() {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "crateful") {
            proj_dirs.config_dir();
        }
    }

    //        unsafe { env::set_var("INCOMING_PATH", path) }
    pub fn read_incoming_path() -> Result<String, VarError> {
        dbg!(env::var("INCOMING_PATH").unwrap());
        env::var("INCOMING_PATH")
    }

    pub fn set_env(key: &str, value: &str) {
        if let Ok(lines) = read_lines("../../dev/crateful/.env") {
            dbg!("oh hell");
            for line in lines.map_while(Result::ok) {
                dbg!("{}", line.clone());
                dbg!("{}", line);
            }
        }
    }
}
