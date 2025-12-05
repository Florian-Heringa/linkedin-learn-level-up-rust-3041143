use regex::Regex;

mod run_length_encoding {
    use regex::Regex;

    pub fn encode(text: &str) -> String {

        let mut res = String::new();
        let mut count: usize = 1;
        let mut prev = text.chars().next().unwrap();
        
        for c in text.chars().skip(1) {
            if c == prev && count < 9 {
                count += 1;
            } else {
                res.push_str(&count.to_string());
                res.push(prev);
                prev = c;
                count = 1;
            }
        }
        // Handle final character
        res.push_str(&count.to_string());
        res.push(prev);
        res
    }
    
    pub fn decode(text: &str) -> String {
        let mut result = String::new();
        let re = Regex::new(r"(?<amount>\d)(?<character>[\d\w\s])").unwrap();
        for caps in re.captures_iter(text) {
            result.push_str(
                &[caps.name("character").unwrap().as_str()]
                    .repeat(caps.name("amount").unwrap().as_str().parse::<usize>().unwrap())
                    .join("")
                )
        }
        result
    }
}

fn main() {
    use run_length_encoding::*;
    let s = "5A1 9A1A1 9A9A2A";
    println!("{:?}", decode(s));
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
