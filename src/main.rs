#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::error::Error;
use async_openai::{types::CreateCompletionRequestArgs, Client};
use clap::Parser;
use tokio::process::Command;
use std::env;

#[derive(Parser)]
#[command(author="Rijul Ranjan", version, about="Why do all the work when AI can instead?", long_about = None)]
struct Cli {
    /// The prompt given by the user
    prompt: Vec<String>,

    /// Execute generated text, such as shell commands
    #[clap(short = 'e', long)]
    exec: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::parse();

    println!("Prompting...");

    let prompt: &str = args.prompt.first().map(String::as_str).unwrap_or_default();

    let client: Client = Client::new();

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(prompt)
        .max_tokens(256_u16)
        .temperature(0.0)
        .top_p(1.0)
        .best_of(1)
        .logprobs(1)
        .echo(true)
        .frequency_penalty(0.0)
        .presence_penalty(0.0)
        .build()?;

    let response = client.completions().create(request).await?;

    let choice = response.choices.iter().nth(0).unwrap();

    if args.exec {
        
        let shell = env::var("SHELL").unwrap_or_default();

        let mut command = Command::new(shell);
        command.arg("-c").arg(&choice.text);

        let output = command.output().await?;
        
        println!("{}", String::from_utf8_lossy(&output.stdout));

    } else {

        println!("{}", choice.text);

    } 

    Ok(())
}
