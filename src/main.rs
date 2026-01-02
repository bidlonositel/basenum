use std::{process::exit, env::args};

const VERSION: &'static str = "0.1";

fn print_help() {
    println!("basenum ver {VERSION}.\n");
    println!("A simple program that converts a number into a specific number system.");
    println!("By default, this program uses a binary number system.");
    println!("To specify a number system, you can use a -ns flag.");

}

// check all arguments and return index of number
fn arg_check(args: &Vec<String>) -> (usize, Option<u64>) {
    let mut index: usize = 1;

    let mut number_system: Option<u64> = None;
    for arg in args {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                exit(0);
            }

            "-ns" | "--number-system" => {
                number_system = {
                    Some(args[2].trim().parse().expect(&format!("{}: Couldn't parse a number. (u64)", args[2])))
                };
                index = 3;
            }

            _ => {}
        }
    }

    return (index, number_system);
}

// convert a number
fn to_base(mut num: u64, number_system: u64) -> String {
    if num == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    while num > 0 {
        let remainder = (num % number_system as u64) as usize;
        result.push(digits.chars().nth(remainder).unwrap());
        num /= number_system as u64;
    }
    result.chars().rev().collect()
}

fn main() {
    let arg_list = args().collect::<Vec<String>>();
    if arg_list.len() == 1 { print_help() ; exit(1) }

    let num_index = arg_check(&arg_list).0;
    let number_system = arg_check(&arg_list).1;

    let num: u64 = arg_list[num_index].trim().parse().expect(&format!("{}: Couldn't parse a number (i64)", arg_list[1]));
    match number_system {
        Some(2) => println!("{num:b}"),

        Some(value) => {
            println!("{}", to_base(num, value));
        },

        std::option::Option::None => {
            println!("{num:b}");
        }
    }
}
