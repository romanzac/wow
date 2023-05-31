mod controller;
mod driver;

use crate::driver::settings::{init_logger, parse_config};
use clap::Parser;
use controller::tcp_server::WowServer;

#[derive(Parser, Debug)]
#[command(version)]
struct ServerArgs {
    /// Config file path
    #[arg(short, long, default_value = "./wserver/config/wow.toml")]
    cfg_file: String,
}

fn main() {
    let args: ServerArgs = Parser::parse();

    let cfg = parse_config(&args.cfg_file).unwrap();

    init_logger(&cfg.log_level);

    // Start new server instance
    WowServer::new(cfg).start();
}
