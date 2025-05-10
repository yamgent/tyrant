mod app;
mod command_bar;
mod core;
mod font;
mod oswin;
mod ui_text;

use anyhow::Result;

pub fn run() -> Result<()> {
    app::App::run_app()
}
