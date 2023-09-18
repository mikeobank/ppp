# PPP

Rust CLI tool to deterministically map a list of strings (passwords) into a 256 bit seed.
This is highly insecure! Do not use this for any real world value or data. Though, the interesting question is if there is a treshold of number of passwords, character sets and password length where brute forcing becomes hard enough?

## docs
In psuedocode this is happening:
```
256 bits entropy: strings.reduce (entropy, string, index) => 
    pbkdf2 (password: string, salt: entropy, iterations: 512 * (index + 1), hash-function: HMAC SHA256)
```

## run
```
cargo run password asdfghjkl 1984
```

## todo
- character sets, entropy calculation
- dictionary attack checks
