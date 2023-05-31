use std::io::{Read, Write};
use std::net::TcpStream;

use argon2::{self, Config, ThreadMode, Variant};
use clap::Parser;
use rand::{distributions::Alphanumeric, Rng};

use common::{parse_msg_in, prepare_msg_out, Quote};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ClientArgs {
    /// Server IP address
    #[arg(short, long, default_value = "0.0.0.0")]
    srv_ip: String,
    /// Server port number
    #[arg(short, long, default_value = "3333")]
    port: String,
}

fn main() {
    // Prepare Argon configuration
    let argon_config = Config::<'_> {
        variant: Variant::Argon2d,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        mem_cost: 65536,
        hash_length: 32,
        ..Default::default()
    };

    // Parse command line args
    let args: ClientArgs = Parser::parse();

    let srv_address = args.srv_ip + ":" + &args.port;

    // Connect to the server
    let mut stream = TcpStream::connect(&srv_address).unwrap();
    println!("Successfully connected to server at {:?}", &srv_address);

    // Send quote-request to the server (10 times allowed without a work)
    let (_, bytes_out) = prepare_msg_out("quote-request", "");
    stream.write_all(&bytes_out).unwrap();

    let mut buf = [0u8; 10000];

    loop {
        match stream.read(&mut buf) {
            Ok(size) => {
                // Check if the message is one of the supported
                let message = match parse_msg_in(&buf, size) {
                    Ok(m) => m,
                    Err(e) => {
                        let (_, bytes_out) = prepare_msg_out("bad-message-error", &e.to_string());
                        stream.write_all(&bytes_out).unwrap();
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .unwrap_or_default();
                        break;
                    }
                };

                // Act based on protocol message
                match &*message.msg {
                    // Get challenge from the sever and return hash
                    "challenge-response" => {
                        // Generate salt
                        let salt: String = rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(8)
                            .map(char::from)
                            .collect();

                        // Hash the password
                        let hash = argon2::hash_encoded(
                            message.data.as_ref(),
                            &salt.clone().into_bytes(),
                            &argon_config,
                        )
                        .unwrap()
                        .to_string();

                        let (_, bytes_out) = prepare_msg_out("hash-request", &hash);
                        stream.write_all(&bytes_out).unwrap();
                    }

                    // Parse and display received quote
                    "quote-response" => {
                        let quote = serde_json::from_str::<Quote>(&message.data).unwrap();
                        println!("Received quote:\n {:?}", quote.text);
                        println!("Author: \n {:?}", quote.author);
                        break;
                    }

                    _ => {
                        break;
                    }
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                break;
            }
        }
    }
}
