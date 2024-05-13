use std::{env, fs};
use colored::*;
use regex::Regex;


#[derive(Debug)]
struct Arguements<'a> {
    pattern: &'a String,
    input_file: &'a String,
    output_file: &'a String,
    replace: &'a String,
}

fn main(){

    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    run(argc, argv)
}

fn run(argc: usize, argv: Vec<String>){
    let args = parse_args(argc, &argv);
    let data = read_file_content(&args.input_file);

    let replaced_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(data)=>data,
        Err(_) => {
            eprintln!("{} Failed to replace data!", "Error".red());
            std::process::exit(1);
        }
    };

    write(&args.output_file, &replaced_data);
}

fn parse_args(argc: usize, argv: &Vec<String>) -> Arguements{
    println!("{}",argc);//Simulating logging
    println!("{}",argv.len());

    if argc != 5 {
        eprintln!("{} Wrong number of arguements!","ERROR".red());
        std::process::exit(400);
    }

    return  Arguements{
        pattern: &argv[1],
        replace: &argv[2],
        input_file: &argv[3],
        output_file: &argv[4],
    }
}

fn read_file_content(file_path: &str) -> String{
    match fs::read_to_string(&file_path) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("{} Failed to read from file {}", "Error".red(), &file_path);
            std::process::exit(1);
        }
    }
}

fn replace(search_pattern: &str, replacement: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(search_pattern)?;

    Ok(regex.replace_all(data, replacement).to_string())
}

fn write(output_file: &str, data: &str){
    match fs::write(&output_file, &data) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("{} Failed to write to file {}", "Error".red(), &output_file);
            std::process::exit(1);
        }
    }
}