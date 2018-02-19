use std::io::{self,BufRead};
use std::path::Path;
use std::fs::File;
use std::env;
use std::process;

fn read_file<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn ciphers_max_length(ciphers: &Vec<String>) -> usize {
    let mut length = 0;
    for cipher in ciphers {
        if length < cipher.chars().count() {
            length = cipher.chars().count();
        }
    }
    length
}

fn space_string(len: usize) -> String {
    let mut i = 0;
    let mut str = String::new();
    while i < len as u32 {
        str.push('2');
        str.push('0');
        i = i + 1;
    }
    str
}

fn is_alpha(char: char) -> bool {
    (char >= 'A' && char <= 'Z') || (char >= 'a' && char <= 'z')
}

fn is_ascii_printable(char: char) -> bool {
    (char >= ' ' && char <= '~') 
}

fn decode_hex(encoded: &String) -> Vec<u8> {
    let chars: Vec<char> = encoded.chars().collect();
    let split = chars
      .chunks(2)
      .map(|chunk| chunk.iter().collect::<String>())
      .collect::<Vec<_>>();

    // Make vector of bytes from octets
    let mut bytes = Vec::new();
    for i in split {
        let res = u8::from_str_radix(&i, 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => println!("Problem with hex: {}", e),
        };
    };
    bytes
}

fn xor_strings(str1: &String, str2: &String) -> String {
    let str1_dec = decode_hex(str1);
    let str2_dec = decode_hex(str2);
    str1_dec
     .iter()
     .zip(str2_dec.iter())
     .map(|(a, b)| (a ^ b) as char)
     .collect()
} 

fn get_key(ciphers: Vec<String>) -> String {
    let max_length = ciphers_max_length(&ciphers);
    let mut known_key : Vec<String> = vec![String::from("00"); max_length];
    let space_string = space_string(max_length);
    for (i, current_cipher) in ciphers.iter().enumerate() {
        let mut count_spaces = vec![0; max_length]; 
        let mut known_spaces = Vec::new();
        for (j, cipher) in ciphers.iter().enumerate() {
            if j != i {
                let str = xor_strings(current_cipher, cipher);
                for (index_char, char) in str.chars().enumerate() {
                    if is_alpha(char) {
                       count_spaces[index_char] += 1;
                    }   
                }
            } 
        }    

        //Store spaces positions
        for (j, val) in count_spaces.iter().enumerate(){
            if val >= &7 {
               known_spaces.push(j); 
            }
        }        
        
        let xor_with_spaces : Vec<String> = xor_strings(current_cipher, &space_string)
          .chars()
          .map(|c| format!("{:02x}", c as u8))
          .collect();
        for pos in known_spaces {
            known_key[pos] = xor_with_spaces[pos].clone();
        }
    }
    known_key.concat()
}

fn unencrypt(target: &String, key: &String) -> String {
    xor_strings(target, key)
}

fn print_text(text: String) {
    print!("Plain text: ");
    for c in text.chars() {
        if is_ascii_printable(c) {
            print!("{}", c);    
        } else {
            print!("*");
        }
    }
    println!("");
}

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage:");
        println!("\t$ ./many-time-pad CIPHERS_FILE TARGET_CIPHER_FILE\n");
        process::exit(1);
    } 
    
    let ciphers_path = &args[1];
    let target_path = &args[2];
    let ciphers = read_file(ciphers_path);
    let target = &read_file(target_path)[0];

    let key = &get_key(ciphers);
    let plain_text = unencrypt(target, key);
    print_text(plain_text);
}
