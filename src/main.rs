mod app;
mod terminal;
mod ui;
mod config;
mod utils;

fn main() -> anyhow::Result<()> {
    app::run()
}