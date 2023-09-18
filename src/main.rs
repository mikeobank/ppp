use pbkdf2::{pbkdf2_hmac_array};
use sha2::Sha256;
use bip39::{Mnemonic, Language};

fn main() {

    const SIZE: usize = 32; // 256 / 8
    const N: u32 = 512;

    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Supply at least one \"password\"");
        std::process::abort();
    }

    let mut bytes: [u8; SIZE] = [0; SIZE];

    for (i, arg) in args.iter().skip(1).enumerate() {
        println!("{}", arg);
        let s = arg.trim();
        let password = s.as_bytes();
        let salt = &bytes;
        let i_u32: u32 = i.try_into().unwrap();
        let n: u32 = N * (i_u32 + 1);
        bytes = pbkdf2_hmac_array::<Sha256, SIZE>(password, salt, n);
    }

    println!("entropy: {:?}", bytes);

    let mnemonic = Mnemonic::from_entropy(&bytes, Language::English).unwrap();
    println!("mnemonic: {}", mnemonic);
}
