use directories::{BaseDirs, ProjectDirs};
use dotenv;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
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
    let file = File::open(filename).expect("well");
    Ok(io::BufReader::new(file).lines())
}
impl Envs {
    // develop & debug loads project local env
    //    #[cfg(debug_assertions)]
    pub fn load_envs() {
        match ProjectDirs::from("", "", "crateful") {
            Some(proj_dirs) => {
                let my_linux_path = proj_dirs.config_dir().to_str().unwrap().to_string();
                let with_env = format!("{}/.env", my_linux_path);
                dotenv::from_path(with_env).ok();
            }
            None => {}
        }
    }

    //    #[cfg(all(not(debug_assertions), target_os = "linux"))]
    //    pub fn load_envs() {
    //        #[cfg(all(not(debug_assertions), target_os = "linux"))]
    //        let my_linux_path = env::home_dir()
    //            .and_then(|a| Some(a.join("/.config/crateful/.env")))
    //            .unwrap();
    //        dotenv::from_path(my_linux_path.as_path());
    //    }
    //
    pub fn try_config_load() {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "crateful") {
            proj_dirs.config_dir();
        }
    }

    pub fn read_env_var(var: String) -> Result<String, env::VarError> {
        env::var(var)
    }

    pub fn set_env(key: &str, value: &str) {
        let mut to_write: Vec<String> = Vec::new();
        let newpair = format!("{}={}\n", key, value);

        let env_path;
        if let Some(env) = ProjectDirs::from("", "", "crateful") {
            env_path = env.config_dir().to_str().unwrap().to_string();
            let with_file = format!("{}/.env", env_path);
            let mut env_file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&with_file)
                .unwrap();
            if let Ok(lines) = read_lines(with_file) {
                for line in lines.map_while(Result::ok) {
                    if line.is_empty() {
                        return;
                    } else if line.contains(&key) {
                        //                    env_file.write(newpair.clone().as_bytes()).unwrap();
                        to_write.push(newpair.clone());
                    } else {
                        let liner = format!("{}\n", line);
                        // env_file.write(liner.as_bytes()).unwrap();
                        to_write.push(liner);
                    };
                }
            }
            for line in to_write {
                let _ = env_file.write(line.as_bytes());
            }
        }
    }
    pub fn create_config() {
        fs::create_dir(
            dirs::config_dir()
                .and_then(|a| Some(a.join("crateful")))
                .unwrap()
                .as_path(),
        )
        .unwrap();
        File::create(
            dirs::config_dir()
                .and_then(|a| Some(a.join("crateful/.env")))
                .unwrap()
                .as_path(),
        )
        .unwrap();
        // maybe write empty env variables to this config file?
        if let Some(base_dirs) = BaseDirs::new() {
            let _home_path = base_dirs.home_dir();
            let empty_vars = [
                format!("INCOMING_PATH=\n"), //home_path.to_str().unwrap()),
                //home_path.to_str().unwrap()),
                format!("SAVE_PATH_A=\n",),
                format!("SAVE_PATH_D=\n"),
                format!("SAVE_PATH_G=\n"),
            ];
            let mut env_file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(
                    dirs::config_dir()
                        .and_then(|a| Some(a.join("crateful/.env")))
                        .unwrap(),
                )
                .unwrap();
            for line in empty_vars {
                let _ = env_file.write(line.as_bytes());
            }
        }
    }
}
