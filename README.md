# PPP

Rust CLI tool to deterministically map (key stretch) a list of strings (passwords) into a 256 bit seed.
This is highly insecure! Do not use this for any real world value or data. Though, the interesting question: is there a treshold of number of passwords (character sets and password lengths) where brute forcing becomes hard enough?

## docs
In psuedocode this is happening:
```
256 bits entropy: strings.reduce (entropy, string, index) =>
    pbkdf2 (password: string, salt: entropy, iterations: 600_000 * (2 ^ index), hash-function: HMAC SHA256)
```

## run
Make sure to have Rust and Cargo installed.
```
cargo run password asdfghjkl 1984
```

## to do
- prompt, spinner, key stretching per step
- command line arguments
    - `initial_n`
    - `key_size`
    - `minimum_depth`
    - `hide passwords`
- PBKDF2 vs. Argon2

## resources
- Current password dictionary: https://raw.githubusercontent.com/duyet/bruteforce-database/master/1000000-password-seclists.txt
