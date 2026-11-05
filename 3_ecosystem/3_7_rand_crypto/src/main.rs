use argon2::{
    Argon2, password_hash::PasswordHasher,
};
use std::fs;
use std::io;
use rand::{self, RngExt, distr::Alphanumeric, seq::IndexedRandom};
use sha3::{Digest, Sha3_256};

fn generate_password(chars: Vec<char>, length: usize) -> String {
    let mut rng = rand::rng();

    (0..length)
        .map(|_| *chars.choose(&mut rng).unwrap())
        .collect()

}

fn select_rand_val(slice: &str) -> char {
    let mut rng = rand::rng();
    let chars: Vec<char> = slice.chars().collect();

    chars.choose(&mut rng).unwrap().clone()
}

fn new_access_token() -> String {
    let mut rng = rand::rng();

    (0..64)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

fn get_file_hash(path: &str) -> io::Result<String> {

    let file = fs::read(path)?;
    let result = Sha3_256::digest(&file);

    Ok(hex::encode(result))

}

fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes())?;

    Ok(password_hash.to_string())
}

fn main() {

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_password() {
        let chars: Vec<char> = vec!['a','b','c','d','1','2','3','4'];

        assert_eq!(generate_password(chars, 10).len(), 10);
    }

    #[test]
    fn test_select_rand_val() {
        let chars = "1234567890";

        assert!(chars.contains(select_rand_val(chars)));
    }
}
