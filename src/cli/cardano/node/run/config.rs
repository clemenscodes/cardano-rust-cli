use std::net::IpAddr;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RunConfig {
    #[structopt(short, default_value = "3001", about = "The port the node runs on")]
    port: u16,
    #[structopt(short, parse(from_os_str), about = "The path to the blockchain")]
    db: PathBuf,
    #[structopt(short, default_value = "127.0.0.1")]
    host: IpAddr,
    #[structopt(short, parse(from_os_str))]
    topology: PathBuf,
    #[structopt(short, parse(from_os_str))]
    socket: PathBuf,
    #[structopt(short, parse(from_os_str))]
    config: PathBuf,
}
