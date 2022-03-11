// use libp2p::websocket;
use clap::{Arg, App, SubCommand};
use shell_command;

// const WS_RPC_CLIENT: &str = "ws://api.testnet.solana.com/";

fn main() {
    let matches = App::new("rust-solana-cli")
        .version("0.1.0")
        .author("Soushi888 <sacha.pignot@protonmail.com>")
        .about("Create a Solana Toekn (SPL) on the Solana Testnet and disable the minting.")
        .subcommand(SubCommand::with_name("create-token")
            .about("Create a Solana Toekn (SPL) on the Solana Testnet")
            .arg(Arg::with_name("name")
                .short("n")
                .long("name")
                .required(true)
                .takes_value(true)
                .help("Name of the Token")
            )
        )
        .subcommand(SubCommand::with_name("show-account")
            .about("Show all the tokens of the connected account")
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create-token") {
        create_token(matches.value_of("name").unwrap().to_string());
    } else if let Some(matches) = matches.subcommand_matches("show-account") {
        let show_account_command = shell_command::run_shell_command("spl-token accounts").unwrap();
        println!("Account : {}", show_account_command);
    } else {
        println!("rust-solana-cli is a program for creating a Solana Toekn (SPL) on the Solana Testnet and register its name.");
    }
}

fn create_token(token_name: String) {
    println!("Token name : {:?}\n", token_name);

    let check_if_wallet = shell_command::run_shell_command("solana config get").unwrap();
    println!("{}", check_if_wallet);

    let create_token_command = shell_command::run_shell_command("spl-token create-token").unwrap();
    let mut create_token_result = create_token_command.split(" ");
    let created_token_hash = create_token_result.nth(2).unwrap().to_string().replace("\n\nSignature:", "");
    println!("Created Token Hash : {}", created_token_hash);

    let create_account_commande = shell_command::run_shell_command(&*format!("spl-token create-account {}", created_token_hash)).unwrap();
    let mut create_account_result = create_account_commande.split(" ");
    let created_account_hash = create_account_result.nth(2).unwrap().to_string().replace("\n\nSignature:", "");
    println!("Created Account Hash :  {}\n", created_account_hash);

    let mint_tokens_command = shell_command::run_shell_command(&*format!("spl-token mint {} 10000000 {}", created_token_hash, created_account_hash)).unwrap();
    println!("{}", mint_tokens_command);

    let disable_mint_tokens_command = shell_command::run_shell_command(&*format!("spl-token authorize {} mint --disable", created_token_hash)).unwrap();
    println!("{}", disable_mint_tokens_command);

    let show_balance_command = shell_command::run_shell_command(&*format!("spl-token balance {}", created_token_hash)).unwrap();
    println!("Balance : {}", show_balance_command);
}
