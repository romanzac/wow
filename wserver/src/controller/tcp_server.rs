use crate::controller::tcp_handler::handle_client;
use crate::driver::settings::SeverConfig;
use common::Wow;
use log::{error, info};
use nanoid::nanoid;
use passwords::PasswordGenerator;
use std::net::TcpListener;
use std::thread;

// Type representing server instance
pub struct WowServer {
    cfg: SeverConfig,
    pg: PasswordGenerator,
    quotes: Wow,
}

impl WowServer {
    pub fn new(cfg: SeverConfig) -> Self {
        // Load quotes into memory
        let quotes_str = std::fs::read_to_string(&cfg.quotes_file).unwrap();
        let quotes = serde_json::from_str::<Wow>(&quotes_str).unwrap();

        // Prepare password generator
        let pg = PasswordGenerator::new()
            .length(8)
            .numbers(true)
            .lowercase_letters(true)
            .uppercase_letters(true)
            .symbols(false)
            .spaces(false)
            .exclude_similar_characters(true)
            .strict(true);

        Self { cfg, pg, quotes }
    }

    pub fn start(&self) {
        // Listen and respond to clients requests
        let srv_address = self.cfg.listen.clone() + ":" + &self.cfg.port;
        let listener = TcpListener::bind(&srv_address).unwrap();
        info!("Server listening at {}", &srv_address);

        // Create new thread for each client
        thread::scope(|s| {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        // Create a thread to handle new client
                        s.spawn(|| {
                            let client_id = nanoid!();
                            info!(
                                "New client ID {} from {}",
                                client_id,
                                stream.peer_addr().unwrap()
                            );
                            handle_client(stream, &client_id, &self.pg, &self.quotes)
                        });
                    }
                    Err(e) => {
                        error!("Error reading from the stream: {}", e);
                    }
                }
            }
        });

        drop(listener);
    }
}
