use reqwest::Client;
use std::fs::File;
use std::io::{stdout, Write};
use std::process::exit;
use colored::Colorize;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();
    let key = client.get("https://gist.githubusercontent.com/siinomega/344aa5cd3af8a2dba9b6bfc486ad8dea/raw").send().await?.text().await?;
    let endpoint = "https://api.hunter.io/v2/domain-search?domain=";
    let auth = "&api_key=";
    let full_url = format!("{}{}{}{}", endpoint, args.url, auth, key);
    let response = client.get(&full_url).send().await?.text().await?;
    let msg = "Response : ";
    println!("\n{} {}",msg.green() ,response);
    loop {
        let mut input = String::new();
        print!("Save Results? (y/n): ");
        stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input)?;

        match input.trim() {
            "y" => {
                let mut file = File::create("results.txt")?;
                file.write_all(response.as_bytes())?;
                println!("{}", "File Saved Successfully!".green());
                exit(0);
            },
            "n" => exit(0),
            _ => println!("{}", "Answer With Either y/n".red()),
        }
    }
}
