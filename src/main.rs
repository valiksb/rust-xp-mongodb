mod error;

use clap::{arg, command, ArgAction};
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, this world!");

    let matches = command!() // requires `cargo` feature
        .next_line_help(true)
        .arg(arg!(--mongodb_url <VALUE>).required(true).action(ArgAction::Set))
        .get_matches();

    println!(
        "mongodb_url: {:?}",
        matches.get_one::<String>("mongodb_url").expect("required")
    );

    let mongodb_url = matches.get_one::<String>("mongodb_url").unwrap();

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&mongodb_url, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }
//    let client = Client::with_uri_str(mongodb_url)?;

    Ok(())
}
