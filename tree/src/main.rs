use std::fs;
use std::env::args;

fn print_directory(root_path: &str) {
    let paths = fs::read_dir(root_path).unwrap(); 
    for path in paths {
        let path_s = path.unwrap().path();
        let name = path_s.to_str().unwrap();
        println!("{}", name);
        let md = fs::metadata(&name).unwrap();
        match md.is_dir() {
            false => println!("\t{}", &name),
            true => {
                println!("\t{}", &name);
                print_directory(&name)
            }
        }
    } 
}

fn main() {
    let args: Vec<_> = args().collect();
    
    let path = match args.len() < 2 {
        true => ".",
        false => args[1].as_ref(),
    };
 
    print_directory(path);
}
