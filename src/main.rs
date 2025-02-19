use fips202::shake256;
use rand::Rng;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() <= 2 {
        println!("\nUsage:\n\n mnm `command` `payload` `mode`\n\n`command` — `encode` (for hash payload) or `decode` (mnemonic payload)\n`mode` (optional) — `auto` or if you want to be able to choose your own words, then it's `pick`\n\nAuthor: \x1b]8;;https://github.com/kovalensky\x1b\\kovalensky\x1b]8;;\x1b\\\n");
        exit(0);
    }

    let command = &args[1].to_lowercase();
    let payload = &args[2].to_lowercase();
    let mode = args
        .get(3)
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "auto".to_string());

    if command == "encode" {
        encode(&payload, &mode);
    } else if command == "decode" {
        decode(&payload);
    } else {
        eprintln!("{} {}", format_text("Invalid command:", 196), command);
        exit(1)
    }
}

fn encode(hash: &str, mode: &str) {
    if !hash.chars().all(|c| c.is_ascii_hexdigit()) || hash.len() % 8 != 0 {
        eprintln!("{}", format_text("Invalid info-hash provided", 196));
        exit(1);
    }

    let hash_array: Vec<String> = hash
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    let dictionary_file = "./data/dictionary.json";
    if !fs::metadata(&dictionary_file).is_ok() {
        eprintln!(
            "{} {}",
            format_text("Can't locate the dictionary:", 196),
            &dictionary_file
        );
        exit(1);
    }

    let dictionary_string = fs::read_to_string(&dictionary_file).unwrap_or_else(|_| {
        panic!("Can't read {}", &dictionary_file);
    });

    let dictionary: HashMap<String, Value> = serde_json::from_str(&dictionary_string)
        .unwrap_or_else(|_| {
            eprintln!("{}", format_text("Can't parse the json", 196));
            exit(1);
        });

    let last_element = hash_array.last().unwrap();
    let protocol = match hash.len() {
        40 => "magnet:?xt=urn:btih:",
        64 => "magnet:?xt=urn:btmh:1220",
        _ => "",
    };

    let mut mnemonic_sentence = String::new();
    let mut i = 0;

    for hash_snippet in &hash_array {
        if let Some(value) = dictionary.get(hash_snippet) {
            let words = match value {
                Value::Array(arr) => arr,
                _ => {
                    eprintln!(
                        "{} {}",
                        format_text(&"Can't find the hash snippet in dictionary:", 196),
                        hash_snippet
                    );
                    exit(1);
                }
            };

            let word: &str;
            i += 1;

            if words.len() > 1 {
                if mode == "pick" {
                    let word_count = hash_array.len();
                    print!(
                        "{}",
                        &format!(
                            "Choose a mnemonic word for hash code ({}) | {} out of {}\r\n",
                            hash_snippet,
                            format_text(&format!("#{}", i), 178),
                            word_count
                        ),
                    );

                    for (j, pickup_option) in words.iter().enumerate() {
                        println!("{}) {}", j + 1, pickup_option);
                    }

                    let mut position_str = String::new();
                    print!("{}", format_text(&"\nEnter the number: ", 250));
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut position_str).unwrap();
                    let position = position_str.trim().parse::<usize>().unwrap_or_else(|_| {
                        eprintln!("{}", format_text("Invalid input, exiting...", 196));
                        exit(1);
                    });

                    if position == 0 || position > words.len() {
                        eprintln!("{}", format_text("Invalid choice, exiting...", 196));
                        exit(1);
                    }

                    word = words[position - 1].as_str().unwrap();
                } else {
                    let position = rand::rng().random_range(0..words.len());
                    word = words[position].as_str().unwrap();
                }
            } else {
                word = words[0].as_str().unwrap();
            }

            mnemonic_sentence.push_str(word);

            if hash_snippet != last_element {
                mnemonic_sentence.push('-');
            }
        } else {
            eprintln!(
                "Value in the dictionary is not a key: {}",
                format_text(&format!("{}", hash_snippet), 196)
            );
            exit(1);
        }
    }

    println!(
        "{}{}",
        format_text("Your mnemonic link: ", 250),
        format_text(&format!("{}{}", protocol, mnemonic_sentence), 70)
    );
}

fn decode(mnemonic: &str) {
    let re = Regex::new(r"^[\p{L}]+(-[\p{L}]+)*$").unwrap();

    if !re.is_match(mnemonic) {
        eprintln!("{}", format_text("Invalid mnemonic sentence provided", 196));
        exit(1);
    }

    let mnemonic: Vec<&str> = mnemonic.split('-').collect();
    const OUTPUT_DIGEST_LENGTH: [u8; 2] = [0u8; 2];
    let mut hash = String::new();
    let protocol = match mnemonic.len() {
        10 => "magnet:?xt=urn:btih:",
        16 => "magnet:?xt=urn:btmh:1220",
        _ => "",
    };

    for word in mnemonic {
        let input = &mut word.as_bytes().to_owned();
        let mut inlen = input.len();
        let mut output = OUTPUT_DIGEST_LENGTH;
        let outlen = output.len();

        shake256(&mut output, outlen, input, &mut inlen);
        for byte in &output {
            hash.push_str(&format!("{:02x}", byte));
        }
    }

    println!(
        "{}{}",
        format_text("Recovered hash: ", 250),
        format_text(&format!("{}{}", protocol, hash), 70)
    );
}

fn format_text(text: &str, colour: u32) -> String {
    format!("\x1b[38;5;{}m{}\x1b[0m", colour, text)
}
