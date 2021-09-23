#![feature(format_args_capture)]

use std::env;
use std::process;

mod dict;
use dict::{Mapping, DICT};

fn print_mappings<'a, 'b>(mappings: impl Iterator<Item = &'a Mapping<'a>>) {
    for mapping in mappings {
        println!(" {}", mapping);
    }
}

fn print_dmenu_mappings<'a, 'b>(
        mappings: impl Iterator<Item = &'a Mapping<'a>>) {
    for mapping in mappings {
        let ch = mapping.0;
        for alias in mapping.1.iter() {
            println!("{alias}: {ch}");
        }
    }
}

fn print_help() {
    println!("
xtype [OPTION] [CHAR_NAME]

xtype  — Outputs weird unicode characters

OPTIONS:
  --help        — prints help
  --list        — lists all known chars and their aliases
  --dmenu-list  — prints the list in a more dmenu friendly format

CHAR_NAME:
  Alias for the character to print out.
  If multiple aliases are provided the first one is used.
  Invalid aliases are ignored.");
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
                "--dmenu-list" => {
                    print_dmenu_mappings(DICT.iter());
                    process::exit(0);
                },
                _ => {
                    eprintln!(
                        "Invalid argument \"{}\", use --help for valid options",
                        arg);
                    process::exit(1);
                }
            }
        }

        let arg = arg.trim_end_matches(":");

        for map in DICT.iter() {
            if map.1.contains(&arg.as_ref()) {
                print!("{}", map.0);
                process::exit(0);
            }
        }
    }
    eprintln!("Could not find any symbols");
    process::exit(1);
}
