// use libp2p::websocket;
extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("rust-solana-libp2p")
        .version("0.1.0")
        .author("Soushi888 <sacha.pignot@protonmail.com>")
        .about("Create a Solana Toekn (SPL) on the Solana Testnet and register its name.")
        .subcommand(SubCommand::with_name("create-token")
            .about("Create a Solana Toekn (SPL) on the Solana Testnet")
            .version("0.1.0")
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .required(true)
                .takes_value(true)
                .help("Name of the Token")
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create-token") {
        create_token(matches.value_of("name").unwrap().to_string());
        println!("{:?}", matches)
    }
}

fn create_token(token_name: String) {
    println!("Token creation !");
    println!("Token name : {:?}", token_name);
}
