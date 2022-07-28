use sha2::{Digest, Sha256};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let input = File::open(filename);
    match input {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut hasher = Sha256::new();

            let mut buffer = [0; 1024];
            loop {
                let read = reader.read(&mut buffer);
                match read {
                    Ok(size) => {
                        if size == 0 {
                            break;
                        }
                        hasher.update(&buffer[0..size]);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        break;
                    }
                }
            }

            let hash = hasher.finalize();
            println!("Hash: {:?}", hash);

            let hex_hash = base16ct::lower::encode_string(&hash);
            println!("Hex-encoded hash: {}", hex_hash);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
