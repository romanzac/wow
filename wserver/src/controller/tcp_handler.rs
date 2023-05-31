use argon2::{self};
use common::{parse_msg_in, prepare_msg_out, Wow};
use log::{error, info};
use passwords::PasswordGenerator;
use rand::Rng;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream, client_id: &str, pg: &PasswordGenerator, w: &Wow) {
    let mut challenge = "".to_string();
    let mut quote_request_counter = 0;
    let mut hash_request_counter = 0;

    let mut buf = [0u8; 10000]; // using 10k byte buffer

    loop {
        match stream.read(&mut buf) {
            Ok(size) => {
                // Check if the message is one of the supported
                let message = match parse_msg_in(&buf, size) {
                    Ok(m) => m,
                    Err(e) => {
                        let (msg_out, bytes_out) =
                            prepare_msg_out("bad-message-error", &e.to_string());
                        error!("Sent to {} : {:?}", client_id, &msg_out);
                        stream.write_all(&bytes_out).unwrap();
                        stream
                            .shutdown(std::net::Shutdown::Both)
                            .unwrap_or_default();
                        break;
                    }
                };

                info!("Received from {} : {:?}", client_id, message);

                // Act based on protocol message, challenge value, and quote_request_counter
                match message.msg.as_ref() {
                    "quote-request" => {
                        // Drop client when it sends more than 10 quote requests without providing work
                        if quote_request_counter > 10 {
                            let (msg_out, bytes_out) =
                                prepare_msg_out("repeated-quote-request-error", "");
                            error!("Sent to {} : {:?}", client_id, &msg_out);
                            stream.write_all(&bytes_out).unwrap();
                            break;
                        }

                        // Prepare new challenge and increment quote_request_counter
                        challenge = pg.generate_one().unwrap();

                        let (msg_out, bytes_out) =
                            prepare_msg_out("challenge-response", &challenge.clone());

                        info!("Sent to {} : {:?}", client_id, &msg_out);
                        quote_request_counter += 1;
                        stream.write_all(&bytes_out).unwrap();
                    }

                    "hash-request" => {
                        // Check(soft) if the algo variant used to generate the hash is Argon2d and check the hash
                        if &message.data[..8] == "$argon2d"
                            && argon2::verify_encoded(&message.data, challenge.as_bytes()).is_ok()
                        {
                            // Select one random quote and send it to client
                            let mut rng = rand::thread_rng();
                            let quote = &w.quotes[rng.gen_range(0..w.quotes.len())];
                            let (msg_out, bytes_out) = prepare_msg_out(
                                "quote-response",
                                &serde_json::to_string(quote).unwrap(),
                            );

                            info!("Sent to {} : {:?}", client_id, &msg_out);
                            // Set challenge to empty and counters back to zero, so that client could repeat whole process
                            challenge = "".to_string();
                            quote_request_counter = 0;
                            hash_request_counter = 0;
                            stream.write_all(&bytes_out).unwrap();
                        } else {
                            // When the hash doesn't match, reply with invalid-hash-error
                            let (msg_out, bytes_out) = prepare_msg_out("invalid-hash-error", "");

                            error!("Sent to {} : {:?}", client_id, &msg_out);
                            hash_request_counter += 1;
                            stream.write_all(&bytes_out).unwrap();

                            // Drop client when it sends more than 10 invalid hashes without putting any work
                            if hash_request_counter > 10 {
                                break;
                            }
                        }
                    }

                    _ => {}
                }
            }
            Err(_) => {
                error!(
                    "Terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                break;
            }
        }
    }
}
