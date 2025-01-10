use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufRead;
use std::{fs, io};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

fn main() {
    // 秘密鍵と公開鍵を読み込む
    let private_key = fs::read_to_string(
        "/home/libra/ghq/github.com/smirror/finatext-2025/JWT/jwts2.sec.key",
    )
    .expect("Failed to read public key");

    let file =
        File::open("/home/libra/ghq/github.com/smirror/finatext-2025/JWT/jwts.rand2.txt")
            .expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut failed_count = 0;

    for line in reader.lines() {
        let token = line.unwrap();
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(private_key.as_ref()),
            &validation,
        );

        if token_data.is_err() {
            failed_count += 1;
            continue;
        }

        println!("Decoded JWT: {:?}", token_data.unwrap().claims);
    }

    println!("Failed to decode {} tokens", failed_count);
}
