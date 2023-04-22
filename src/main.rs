#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::error::Error;
use async_openai::{types::CreateCompletionRequestArgs, Client};
use clap::Parser;
use tokio::process::Command;

#[derive(Parser)]
#[command(author="Rijul Ranjan", version, about="ChatGPT in the terminal, 'nuff said", long_about = None)]
pub(crate) struct Cli {
    /// The prompt given by the user
    prompt: Vec<String>,

    /// Execute generated text, such as shell commands
    #[clap(short = 'y', long)]
    force: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::parse();

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

    if args.force {
        
        let mut command = Command::new("bash");
        command.arg("-c").arg(&choice.text);
        let output = command.output().await?;
        println!("{}", String::from_utf8_lossy(&output.stdout));

    } else {

        println!("{}", choice.text);

    } 

    Ok(())
}
