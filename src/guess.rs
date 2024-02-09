use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientGuess {
    pub data: u8
}

#[derive(Serialize, Deserialize, Debug)]
struct Guess {
    message: String,
    rand: u8,
}

pub fn check_guess(guess: u8) -> Vec<u8> {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);
    let res = if guess == secret_number {
        "equal"
    } else {
        "not equal"
    }.to_string(); // Convert to String
    
    let guess = Guess {
        message: res, 
        rand: secret_number,
    };
    
    let serialized = serde_json::to_string(&guess).unwrap(); 

    return serialized.into_bytes();
}