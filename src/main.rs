mod entropy;
mod text_file;

use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;
use bip39::{Mnemonic, Language};
use num_bigint::BigUint;
use num_traits::One;

fn main() {

    const SIZE: usize = 32; // 256 / 8
    const N: u32 = 512;

    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Supply at least one \"password\"");
        std::process::abort();
    }

    let mut bytes: [u8; SIZE] = [0; SIZE];
    let mut total_entropy: BigUint = One::one();

    for (i, arg) in args.iter().skip(1).enumerate() {
        let s = arg.trim();
        let entropy = entropy::calculate_for_string(s);
        println!("{} ({})", arg, entropy);
        total_entropy = total_entropy * entropy;
        let password = s.as_bytes();
        let salt = &bytes;
        let i_u32: u32 = i.try_into().unwrap();
        let b_u32: u32 = 2;
        let n: u32 = N * b_u32.pow(i_u32);
        bytes = pbkdf2_hmac_array::<Sha256, SIZE>(password, salt, n);
    }

    println!("===");
    println!("total ({} = {} bits)", total_entropy, entropy::decimal_to_bits(&total_entropy));

    println!("entropy: {:?}", bytes);

    let mnemonic = Mnemonic::from_entropy(&bytes, Language::English).unwrap();
    println!("mnemonic: {}", mnemonic);
}
