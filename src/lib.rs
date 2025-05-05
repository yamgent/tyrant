mod app;
mod core;
mod oswin;

use anyhow::Result;

pub fn run() -> Result<()> {
    app::App::run_app()
}
