use crate::Argon2;

pub fn hash_key(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut okm = [0u8; 32];
    let _ = Argon2::default().hash_password_into(password, salt, &mut okm);
    okm
}
