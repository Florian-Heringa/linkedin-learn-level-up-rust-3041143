mod vigenere {
    pub fn encrypt(plaintext: &str, key: &str) -> String {       
        // Only works for input strings with uppercase characters only 
        let shifts: Vec<u8> = key.chars().map(|c| c as u8 - b'A').collect();
        let mut encoded = String::with_capacity(plaintext.len());
        let mut key_idx = 0;
        for c in plaintext.chars() {

            let new_char = if c.is_ascii_alphabetic() {
                // Convert to number in range 0..25 
                let char_as_num = c as u8 - b'A';
                // Shift by key amount, keeping in alphabet range
                let shifted = (char_as_num + shifts[key_idx % shifts.len()]) % 26;
                key_idx += 1;
                // Convert back to character
                (shifted + b'A') as char
            } else {
                c
            };

            encoded.push(new_char);
        };

        encoded
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        // Decrypting is the same as encrypting with the inverse of the key
        // The inverse can be found by taking (26 - key[i])
        // So 'A' -> 'Z', 'B' -> 'Y',
        let key: String = key.chars().map(|c| {
            ((26 - (c as u8 - b'A')) + b'A') as char
        }).collect();
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
