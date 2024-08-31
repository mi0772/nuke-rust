use std::sync::{Arc, Mutex};
use log::{debug, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use crate::engine::database::Database;

mod engine;
mod tests;
mod tcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    // read partition number ad path from environment variables
    let partition_number = std::env::var("PARTITION_NUMBER").unwrap_or("10".to_string()).parse::<u32>().unwrap();
    let path = std::env::var("PATH").unwrap_or("path_to_file".to_string());

    let database = Arc::new(Mutex::new(Database::new(partition_number as u8, path)));

    // Bind del listener alla porta specificata
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    info!("Server listening on port 8080");

    while let Ok((mut socket, _)) = listener.accept().await {
        let database = Arc::clone(&database);

        // Gestisci la connessione in un task separato
        tokio::spawn(async move {
            let mut buffer = [0u8; 1024];

            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => return, // Connessione chiusa
                    Ok(n) => {
                        // Converti i dati ricevuti
                        let request = String::from_utf8_lossy(&buffer[..n]);
                        let request = request.trim().replace('\r', "").replace('\n', "");

                        let command = tcp::Command::from_str(&request);
                        match command {
                            Ok(command) => {
                                debug!("Received command: {:?}", command);

                                if request.eq_ignore_ascii_case("quit") {
                                    println!("Received 'quit' command. Closing connection.");
                                    return ;
                                }

                                // Simula un'operazione sul database
                                let response = tcp::handle_request(command, &database).await;

                                if let Err(e) = socket.write_all(response.as_bytes()).await {
                                    println!("Failed to write to socket; err = {:?}", e);
                                }
                            }
                            Err(e) => {
                                println!("Failed to parse command; err = {:?}", e);
                                continue;
                            }
                        }

                    }
                    Err(e) => {
                        println!("Failed to read from socket; err = {:?}", e);
                    }
                }
            }
        });
    }

    Ok(())
}

async fn handle_request(request: &str, database: &Arc<Mutex<Database>>) -> String {
    let mut db = database.lock().unwrap();
    // Implementa la logica per gestire la richiesta basata sul database
    // Esempio: `request` può essere un comando per operare sul database
    "Operation result".to_string()
}