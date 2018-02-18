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

fn empty_string(len: usize) -> String {
    let mut i = 0;
    let mut str = String::new();
    while i < len as u32 {
        str.push(' ');
        i = i + 1;
    }
    str
}

fn is_alpha(char: char) -> bool {
    (char >= 'A' && char <= 'Z') || (char >= 'a' && char <= 'z')
}

fn xor_strings(str1: &String, str2: &String) -> String {
    str1
     .chars()
     .zip(str2.chars())
     .map(|(a, b)| (a as u8 ^ b as u8) as char)
     .collect()
} 

fn to_chars(str: String) -> Vec<char> {
    let mut vec = Vec::new();
    for i in str.chars() {
        vec.push(i);
    }
    vec
}

fn get_key(ciphers: Vec<String>) -> Vec<char> {
    let max_length = ciphers_max_length(&ciphers);
    let mut known_key : Vec<char> = vec!['*'; max_length];
    let empty_string =  empty_string(max_length);
    for (i, current_cipher) in ciphers.iter().enumerate() {
        //FIX: hardcoded length
        let mut count_spaces = vec![0; max_length]; let mut known_spaces = Vec::new();
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
        
        let xor_with_spaces : Vec<char> = to_chars(xor_strings(current_cipher, &empty_string));
        for pos in known_spaces {
            known_key[pos] = xor_with_spaces[pos];
        }
    }
    known_key
}

fn unencrypt(target: &str, key: Vec<char>) {
    let plain_text: String = target.chars().zip(key).map(|(a, b)| (a as u8 ^ b as u8) as char).collect();
    println!("{}", plain_text);
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
    let key = get_key(ciphers);
    for i in key.iter() {
        println!("{}", i);
    }
    let plain_target = unencrypt(target, key);
}
