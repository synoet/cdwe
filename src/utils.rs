use sha2::{Digest, Sha256};

pub fn get_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());

    return format!("{:?}", hasher.finalize());
}
