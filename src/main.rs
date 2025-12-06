mod vigenere {
    use regex::Regex;

    fn to_alphabet_index(c: u8) -> u8 {
        c - b'A'
    }

    fn vigenere_step((key, plain): (u8, u8)) -> u8 {
        (key + plain) % 26
    }

    fn to_character(c: u8) -> char {
        (c + b'A') as char
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        // Clean input, remove all whitespace characters
        let plaintext = Regex::new(r"\s+")
            .unwrap()
            .replace_all(plaintext, "")
            .to_string();

        // Zip cycled key iterator with plaintext and encode while moving through
        key
            .bytes()
            .map(to_alphabet_index)
            .cycle()
            .zip(
                plaintext
                    .bytes()
                    .map(to_alphabet_index)
                )
            .map(vigenere_step)
            .map(to_character)
            .collect()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        // Decrypting is the same as encrypting with the inverse of the key
        // The inverse can be found by taking (26 - key[i])
        // So 'A' -> 'Z', 'B' -> 'Y',
        let key: String = key
            .chars()
            .map(|c| ((26 - (c as u8 - b'A')) + b'A') as char)
            .collect();
        encrypt(ciphertext, &key)
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(ciphertext, key);

    println!("{}", plaintext);
}
