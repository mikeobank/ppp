mod entropy;
mod text_file;
mod key;
mod passwords;

use bip39::{Mnemonic, Language};

fn main() {

    const N: u32 = 600_000;

    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("Supply at least one \"password\"");
        std::process::abort();
    }

    let strings = args[1..].to_vec();
    let passwords = passwords::prepare(strings);

    let key = key::stretch(N, &passwords);
    let (total_entropy, entropies) = entropy::calculate(&passwords);
    let mnemonic = Mnemonic::from_entropy(&key, Language::English).unwrap();

    // print output
    for (i, password) in passwords.iter().enumerate() {
        println!("{} ({})", password, entropies[i]);
    }
    println!("---");
    println!("total ({} = {} bits)", total_entropy, entropy::decimal_to_bits(&total_entropy));
    println!("===");
    println!("entropy: {:?}", key);
    println!("mnemonic: {}", mnemonic);
}
