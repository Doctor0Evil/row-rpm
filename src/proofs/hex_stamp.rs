use sha3::{Digest, Sha3_256};

pub fn generate_hex_stamp<T: serde::Serialize>(data: &T) -> String {
    let bytes = serde_json::to_vec(data).expect("serialize failed");
    let mut hasher = Sha3_256::new();
    hasher.update(&bytes);
    let result = hasher.finalize();
    format!("0x{}", hex::encode(result))
}

pub fn verify_hex_stamp<T: serde::Serialize>(data: &T, stamp: &str) -> bool {
    let expected = generate_hex_stamp(data);
    expected == stamp
}
