use std::net::IpAddr;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
pub enum Args {
    Server,
    Client {
        #[clap(short, long)]
        ip: IpAddr,
    },
}
