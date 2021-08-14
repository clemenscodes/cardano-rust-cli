use cli::Cli;
use ctrlc::set_handler;
use human_panic::setup_panic;
use structopt::StructOpt;
mod cli;

#[tokio::main]
async fn main() {
    setup_panic!();
    Cli::start(Cli::from_args().command).await;
    set_handler(|| println!("Initialize Ctrl-C handler")).expect("Error setting Ctrl-C handler");
}
