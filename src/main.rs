use rand::prelude::*;
use std::io::{self};
use std::env;
use std::fs::File;
use std::io::Read;

const VERSION: &str = "0.1.0";

fn argparse(args: &Vec<String>) -> (usize, String) {
    let mut data: (usize, String) = (128, "dk0319s=1-3595u1pdfjz;cnza0wuf9wq0efjzlkdjq-w91i`ikfsojfpoczldua0idfsof".to_string());
    let mut sel: (bool, &str) = (false, "");

    for arg in args {
        if arg == "-V" || arg == "--version" { // version
            println!("keysmash {}\n", VERSION);
            std::process::exit(0)
        } else if arg == "-i" || arg == "--input" { // input
            if !sel.0 {
                sel = (true, "input"); 
            } else if sel.0 && (sel.1 == "length" || sel.1 == "file") {
                println!("Invalid argument '{}'", arg);
                std::process::exit(128);
            }
        } else if arg == "-l" || arg == "--length" { // length
            if !sel.0 {
                sel = (true, "length");
            } else if sel.0 && (sel.1 == "input" || sel.1 == "file") {
                println!("Invalid argument '{}'", arg);
                std::process::exit(128);
            }
        } else if arg == "-f" || arg == "--file" { // input file (replaces -i)
            if !sel.0 {
                sel = (true, "file");
            } else if sel.0 && (sel.1 == "input" || sel.1 == "length") {
                println!("Invalid argument '{}'", arg);
                std::process::exit(128);
            }
        } else if arg == "-h" || arg == "--help" { // help
            println!("-V (--version) - show version\n
-i (--input) ('-'/string) - specify keysmash seed, '-' if piping from somewhere\n
-l (--length) (number) - specify output length, '128' if not specified\n
-f (--file) (path) - override -i with file contents\n
-h (--help) - displays this message");
            std::process::exit(0);
        } else if sel.0 { // read input
            if sel.1 == "input" {
                if arg == "-" { // from pipe
                    loop {
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("keysmash: read error: failed to read from pipe");
                        input = input.trim().to_string();
                        if input == "" {
                            break;
                        }
                        data.1 = input;
                    }
                } else { // from args
                    data.1 = arg.to_string();
                }
                sel.0 = false;
            } else if sel.1 == "length" { // read length
                let len = arg.parse::<usize>();
                let len = match len {
                    Ok(_t) => len.unwrap(),
                    Err(e) => { println!("Error parsing argument: {}", e); std::process::exit(128); },
                };
                data.0 = len;
                sel.0 = false;
            } else if sel.1 == "file" {
                let mut file = File::open(arg).unwrap();
                let mut contents = String::new();
                let err = format!("keysmash: read error: invalid utf-8 data in file '{}'", &arg[..]);
                file.read_to_string(&mut contents).expect(&err[..]);
                data.1 = contents;
                sel.0 = false;
            }
        }
    }
    return data;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let data = argparse(&args);
    let len: usize = data.0;
    let keys: String = data.1;

    let mut mash: String = String::new().to_owned();
    let mut rng = rand::thread_rng();
    
    for _i in 0..len {
        let tmp = rng.next_u32() as usize % keys.chars().count(); // generate a usize in range [0; LEN)
        let sel: String = keys.chars().nth(tmp).unwrap().to_string(); // select the char
        mash = format!("{}{}", mash, sel);
    }

    println!("{}", mash);
}
