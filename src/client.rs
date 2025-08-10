use std::io::{self, Write};
use std::net::IpAddr;

pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        let base_url = format!("http://{}:{}", ip, port);
        let client = reqwest::Client::new();
        
        Self {
            base_url,
            client,
        }
    }

    pub async fn check_connection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            println!("✓ Connected to server at {}", self.base_url);
        } else {
            return Err(format!("Server responded with status: {}", response.status()).into());
        }
        
        Ok(())
    }

    pub async fn get_state(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/state", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            let state = response.text().await?;
            println!("VM State:");
            println!("{}", state);
        } else {
            println!("Error getting state: {}", response.status());
        }
        
        Ok(())
    }

    pub async fn run_opcode(&self, opcode: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/run", self.base_url);
        let response = self.client
            .post(&url)
            .body(opcode.to_string())
            .send()
            .await?;
        
        if response.status().is_success() {
            let result = response.text().await?;
            println!("✓ {}", result);
        } else {
            let error = response.text().await?;
            println!("✗ Error: {}", error);
        }
        
        Ok(())
    }

    pub async fn interactive_mode(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== MPC Client Interactive Mode ===");
        println!("Connected to: {}", self.base_url);
        println!("Commands:");
        println!("  state           - Get VM state");
        println!("  run <opcode>    - Execute opcode (e.g., 'run 42')");
        println!("  help            - Show this help");
        println!("  quit            - Exit client");
        println!();

        loop {
            print!("mpc> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            match input {
                "quit" | "exit" | "q" => {
                    println!("Goodbye!");
                    break;
                }
                "state" | "s" => {
                    if let Err(e) = self.get_state().await {
                        println!("Error getting state: {}", e);
                    }
                }
                "help" | "h" => {
                    println!("Commands:");
                    println!("  state, s        - Get VM state");
                    println!("  run <opcode>    - Execute opcode (e.g., 'run 42')");
                    println!("  help, h         - Show this help");
                    println!("  quit, exit, q   - Exit client");
                }
                _ => {
                    if input.starts_with("run ") {
                        let opcode_str = input.strip_prefix("run ").unwrap().trim();
                        match opcode_str.parse::<u16>() {
                            Ok(opcode) => {
                                if let Err(e) = self.run_opcode(opcode).await {
                                    println!("Error running opcode: {}", e);
                                }
                            }
                            Err(_) => {
                                println!("Invalid opcode. Please enter a valid number (0-65535).");
                            }
                        }
                    } else {
                        println!("Unknown command: '{}'. Type 'help' for available commands.", input);
                    }
                }
            }
        }

        Ok(())
    }
}

pub async fn run_client(ip: IpAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new(ip, 3000); // Default port from server
    
    // Test connection
    if let Err(e) = client.check_connection().await {
        eprintln!("Failed to connect to server: {}", e);
        eprintln!("Make sure the server is running at {}:3000", ip);
        return Ok(());
    }

    // Start interactive mode
    client.interactive_mode().await?;
    
    Ok(())
}
