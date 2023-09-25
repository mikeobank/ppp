use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;

const SIZE: usize = 32; // 256 / 8
pub type Key = [u8; SIZE];

pub fn stretch(initial_n: u32, passwords: &Vec<String>) -> Key {

  let mut key: Key = [0; SIZE];

  for (i, password) in passwords.iter().enumerate() {
    let bytes = password.as_bytes();
    let salt = &key;
    let i_u32: u32 = i.try_into().unwrap();
    let b_u32: u32 = 2;
    let n: u32 = initial_n * b_u32.pow(i_u32);
    key = pbkdf2_hmac_array::<Sha256, SIZE>(bytes, salt, n);
  }

  return key;
}