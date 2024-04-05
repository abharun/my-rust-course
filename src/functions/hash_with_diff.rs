use sha3::{Digest, Sha3_256};

fn verify_hash(hash: String, difficulty: i32) -> bool {
    let hash_binding = hash.as_str();
    &hash_binding[0..difficulty as usize] == "0".repeat(difficulty as usize)
}

pub fn calculate_hash(text: String, diff: i32) -> ( String, i32 ) {
    let mut hasher = Sha3_256::new();
    let mut nonce = 0;

    hasher.update(text);

    let mut thasher = hasher.clone();
    thasher.update(nonce.to_string());

    let mut hash = format!("{:x}", thasher.finalize());

    while !verify_hash(hash.clone(), diff) {
        thasher = hasher.clone();
        nonce += 1;
        thasher.update(nonce.to_string());
        hash = format!("{:x}", thasher.finalize());
    }

    (hash, nonce)
}