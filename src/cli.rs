use std::net::IpAddr;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
pub enum Args {
    Server {
        #[clap(short, long)]
        #[clap(default_value = "127.0.0.1")]
        ip: IpAddr,
        #[clap(default_value = "3000")]
        port: u16
    },
    Client {
        #[clap(short, long)]
        #[clap(default_value = "127.0.0.1")]
        ip: IpAddr,
        #[clap(default_value = "3000")]
        port: u16
    },
}
