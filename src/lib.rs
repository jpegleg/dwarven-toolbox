#[cfg(test)]
mod tests {
    #[test]
    fn uuidtest() {
      use uuid::Uuid;
      assert_eq!(Uuid::new_v4().to_string().is_empty(), false);
    }

    #[test]
    fn datetest() {
      use chrono::prelude::*;
      assert_eq!(Utc::now().to_string().is_empty(), false);
      let dt_nano = NaiveDate::from_ymd_opt(2014, 11, 28).unwrap().and_hms_nano_opt(12, 0, 9, 1).unwrap().and_local_timezone(Utc).unwrap();
      assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");
    }

    #[test]
    fn envset() {
       use std::env;
       use uuid::Uuid;
       let txid = Uuid::new_v4().to_string();
       env::set_var("DWARF", &txid);
    }
      
    #[allow(unused_imports)]
    #[test]
    fn a2() {
       use std::fs::File;
       use std::io::{self, Read, Write};
       use rand::Rng;
       use rand::SeedableRng;
       use rand::rngs::StdRng;
       let mut key_file = File::open("keytest1.bin").expect("Failed to open the keytest.bin file.");
       let mut key_data = Vec::new();
       let _ = key_file.read_to_end(&mut key_data);
       let mut rng = StdRng::from_entropy();
       let mut nonce = [0u8; 24];
       rng.fill(&mut nonce);
       let bnon = hex::encode(&nonce);
       let binding = "00000000".to_owned() + &bnon;
       let salt = binding.as_bytes();
       use argon2::Argon2;
       let mut okm = [0u8; 32];
       let _ = Argon2::default().hash_password_into(&key_data, salt, &mut okm);
       assert_eq!(okm.is_empty(), false);
    }

   #[allow(unused_imports)]
   #[test]
   fn b3() {
       use hex;
       use std::fs::File;
       use std::io::{self, Read, Write};
       use rand::Rng;
       use rand::SeedableRng;
       use rand::rngs::StdRng;
       let mut key_file = File::open("keytest1.bin").expect("Failed to open the keytest.bin file.");
       let mut key_data = Vec::new();
       let _ = key_file.read_to_end(&mut key_data);
       use blake3::Hasher;
       let mut hasher = Hasher::new();
       hasher.update(&key_data);
       let hashed_key = hasher.finalize();
       let hk = hex::encode(hashed_key.as_bytes());
       assert_eq!(hk, "445816183a03a9249ad0064609daf14f8a02afe378713bb6c02fd801254be5f5");
    }
}
