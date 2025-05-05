mod app;

use anyhow::Result;

pub fn run() -> Result<()> {
    app::App::run_app()
}
