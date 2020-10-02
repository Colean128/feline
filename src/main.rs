use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        println!("Feline by Colean128. Input a file location.")
    } else {
        let file = &args[1];

        let mut string = fs::read_to_string(file)
            .expect("The file could not be read.");

        let len = string.len();
        string.truncate(len - 1);
        println!("{}", string);
    }
}
