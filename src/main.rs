use std::env;
use std::process;

mod dict;
use dict::{Mapping, DICT};

fn print_mappings<'a, 'b>(mappings: impl Iterator<Item = &'a Mapping<'a>>) {
    for mapping in mappings {
        println!(" {}", mapping);
    }
}

fn print_help() {
    println!("
xtype  - types weird characters into the current X window

xtype --help  - prints help
xtype --list  - lists all known chars and their aliases

xtype CHAR_NAME

CHAR_NAME:");
    print_mappings(DICT.iter());
}

fn main() {
    let mut args = env::args();
    args.next();

    for arg in args {
        if arg.starts_with("--") {
            match arg.as_ref() {
                "--help" => {
                    print_help();
                    process::exit(0);
                },
                "--list" => {
                    print_mappings(DICT.iter());
                    process::exit(0);
                }
                _ => {
                    eprintln!(
                        "Invalid argument \"{}\", use --help for valid options",
                        arg);
                    process::exit(1);
                }
            }
        }

        for map in DICT.iter() {
            if map.1.contains(&arg.as_ref()) {
                println!("{}", map.0);
                process::exit(0);
            }
        }
    }
    eprintln!("Could not find any symbols");
    process::exit(1);
}
