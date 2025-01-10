use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
}

fn main() {
    // 秘密鍵と公開鍵を読み込む
    // let private_key =
    //     fs::read_to_string("jwts2.sec.key").expect("Failed to read private key");
    let public_key = fs::read_to_string("/home/libra/ghq/github.com/smirror/finatext-2025/JWT/jwt/src/jwts2.sec.key").expect("Failed to read public key");

    let file = File::open("/home/libra/ghq/github.com/smirror/finatext-2025/JWT/jwt/src/jwts.rand.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    let mut failed_count = 0;

    for line in reader.lines() {
        let token = line.unwrap();
        // 公開鍵を使ってJWTトークンを検証
        let decoded_token = decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
            &Validation::new(Algorithm::RS256),
        );

        if let Err(err) = decoded_token {
            println!("Failed to decode token: {}", err);
            failed_count += 1;
            continue;
        }

        println!("Decoded JWT: {:?}", decoded_token.unwrap().claims);
    }

    println!("Failed to decode {} tokens", failed_count);
}
