mod agent;

use agent::{Agent, AgentConfig};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let id = Agent::create(&mut AgentConfig {
        name: "Galatea".to_owned(),
    });
    let agent = Agent::get().unwrap();

    let mut input = String::new();
    loop {
        print!("User: ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        if input.as_str() == "!quit" { break }

        let response = agent.respond(&input).await;
        println!("\nAgent: {}\n", response);
    }
}