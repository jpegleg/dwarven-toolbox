use argon2::Argon2;
use blake3::Hasher;

#[allow(dead_code)]
pub fn hash_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut okm = [0u8; 32];
    let _ = Argon2::default().hash_password_into(password, salt, &mut okm);
    okm
}

#[allow(dead_code)]
pub fn b3(key_data: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(key_data);
    let hashed_key = hasher.finalize();
    *hashed_key.as_bytes()
}
