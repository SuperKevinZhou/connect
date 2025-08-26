use sha2::{Sha512, Digest};

pub fn compute_sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    
    hasher.update(input.as_bytes());
    
    let result = hasher.finalize();
    
    result.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect()
}