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
       let key_data: Vec<u8> = [55, 99, 53, 102, 51, 48, 52, 50, 55, 97, 50, 49, 53, 49, 50, 52, 52, 57, 50, 101, 53, 53, 51, 55, 51, 102, 53, 51, 54, 56, 53, 101, 53, 57, 54, 51, 50, 53, 54, 52, 55, 101, 53, 51, 50, 102, 53, 100, 51, 55, 55, 48, 52, 50, 50, 56, 50, 48, 50, 53, 55, 101, 50, 49].to_vec();
       let binding = "00000000ffff8888ffff".to_owned();
       let salt = binding.as_bytes();
       use argon2::Argon2;
       let mut okm = [0u8; 32];
       let _ = Argon2::default().hash_password_into(&key_data, salt, &mut okm);
       assert_eq!(hex::encode(okm), "13e247a3039e9499072cb57dfb4ad9283e5ffa7f8306b8945d3d8e8b2f235751");
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
       let key_data: Vec<u8> = [55, 99, 53, 102, 51, 48, 52, 50, 55, 97, 50, 49, 53, 49, 50, 52, 52, 57, 50, 101, 53, 53, 51, 55, 51, 102, 53, 51, 54, 56, 53, 101, 53, 57, 54, 51, 50, 53, 54, 52, 55, 101, 53, 51, 50, 102, 53, 100, 51, 55, 55, 48, 52, 50, 50, 56, 50, 48, 50, 53, 55, 101, 50, 49].to_vec();
       use blake3::Hasher;
       let mut hasher = Hasher::new();
       hasher.update(&key_data);
       let hashed_key = hasher.finalize();
       let hk = hex::encode(hashed_key.as_bytes());
       assert_eq!(hk, "ea534ab07986b9e1c52f9b2d5d811c4dcb368589c03fa9dfe9844ae78cfcd0a3");
    }
}
