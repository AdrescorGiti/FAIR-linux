mod app;
mod dns;
mod error;
mod hosts;
mod instance;
mod network;
mod services;
mod ui;

fn main() -> error::Result<()> {
    ui::run_ui()
}