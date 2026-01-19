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

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    init_cli_log!();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
