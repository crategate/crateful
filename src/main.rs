use crate::app::App;
use cli_log::*;

pub mod app;
pub mod env;
pub mod event;
pub mod instructs;
pub mod keys;
pub mod pause;
pub mod ui;
pub mod volume;

/// ALSA (reached through rodio/cpal) prints messages like
/// "ALSA lib pcm.c:...(snd_pcm_recover) underrun occurred" directly to stderr
/// from its own worker threads whenever the audio buffer starves (e.g. while
/// scrubbing). Because the process is in raw terminal mode, that output tears
/// up the ratatui frame. ALSA's default error handler can't be replaced from
/// stable Rust (it requires a C-variadic callback), so instead we redirect
/// file descriptor 2 to a log file: the terminal stays clean in every build
/// profile, and the messages are still preserved for debugging.
#[cfg(unix)]
fn redirect_stderr_to_log() {
    use directories::ProjectDirs;
    use std::fs::{self, OpenOptions};
    use std::os::unix::io::AsRawFd;

    let Some(proj_dirs) = ProjectDirs::from("", "", "crateful") else {
        return;
    };
    if fs::create_dir_all(proj_dirs.config_dir()).is_err() {
        return;
    }
    let Ok(log_file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(proj_dirs.config_dir().join("crateful.log"))
    else {
        return;
    };
    // Atomically point fd 2 (stderr) at the log file.
    unsafe { libc::dup2(log_file.as_raw_fd(), libc::STDERR_FILENO) };
}

#[cfg(not(unix))]
fn redirect_stderr_to_log() {}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    init_cli_log!();
    color_eyre::install()?;
    redirect_stderr_to_log();
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
