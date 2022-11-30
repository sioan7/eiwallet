use ssi::jwk::JWK;

pub use ssi;

pub fn generate_ed25519_key() -> String {
    // TODO: handle errors
    let jwk = JWK::generate_ed25519().unwrap();
    serde_json::to_string(&jwk).unwrap()
}

#[cfg(test)]
mod test {
    use crate::generate_ed25519_key;

    #[test]
    fn test_generate_ed25519_key() {
        let key = generate_ed25519_key();
        println!("{key}");
    }
}
