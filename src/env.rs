use directories::ProjectDirs;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Envs {
    pub incoming: PathBuf,
    pub save_path_a: PathBuf,
    pub save_path_d: PathBuf,
    pub save_path_g: PathBuf,
}

impl Envs {
    /// Loads the config file (~/.config/crateful/.env) into the process environment.
    pub fn load_envs() {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "crateful") {
            let with_env = proj_dirs.config_dir().join(".env");
            dotenv::from_path(with_env).ok();
        }
    }

    pub fn read_env_var(var: String) -> Result<String, env::VarError> {
        env::var(var)
    }

    /// Writes a key=value pair to the config file, replacing any existing
    /// assignment for that key. The file is rewritten atomically-ish via a
    /// single `fs::write` (which truncates), so a shorter new value can never
    /// leave stale bytes behind. Runtime state lives on `App`, so the running
    /// process does not need its own environment updated — the file is read
    /// again on next startup.
    pub fn set_env(key: &str, value: &str) {
        let Some(proj_dirs) = ProjectDirs::from("", "", "crateful") else {
            return;
        };
        let env_file = proj_dirs.config_dir().join(".env");
        let mut lines: Vec<String> = fs::read_to_string(&env_file)
            .unwrap_or_default()
            .lines()
            .map(str::to_string)
            .collect();
        let newpair = format!("{key}={value}");
        // Match on the exact key before '=', never a substring of the whole line.
        match lines.iter_mut().find(|l| l.split('=').next() == Some(key)) {
            Some(line) => *line = newpair,
            None => lines.push(newpair),
        }
        let _ = fs::write(env_file, lines.join("\n") + "\n");
    }

    /// Creates the config directory and a fresh .env with empty keys. Existing
    /// config files are left untouched.
    pub fn create_config() {
        let Some(config_dir) = dirs::config_dir().map(|d| d.join("crateful")) else {
            return;
        };
        if fs::create_dir_all(&config_dir).is_err() {
            return;
        }
        let env_file = config_dir.join(".env");
        if !env_file.exists() {
            let _ = fs::write(
                env_file,
                "INCOMING_PATH=\nSAVE_PATH_A=\nSAVE_PATH_D=\nSAVE_PATH_G=\n",
            );
        }
    }
}
