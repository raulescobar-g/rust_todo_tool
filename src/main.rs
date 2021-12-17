#[macro_use] extern crate clap;
#[macro_use] extern crate prettytable;
mod packet;
use crate::packet::*;
use std::io::{Error};
use std::collections::{LinkedList,HashSet};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Arg, App}; 


//TODO: add a function that gets files to ignore maybe from the gitignore
//TODO: iterate over files with paths available and get the line numbers
//TODO: maybe print everything in like a tree pattern that mirrors the project structure

//TODO: package app somehow
//TODO: deploy to homebrew
//TODO: cache results on a file but print the actual stuff on the terminal

fn main() -> Result<(), Error>{
 
    let matches = App::new("todo police").version("0.1.0").author("Raul E. <raul3@microanalisis.com>").about("Todo comments aggregator.")
                            .arg(Arg::with_name("silent").short("s").long("silent").help("does not output to console"))
                            .arg(Arg::with_name("outputfile").short("f").long("file").takes_value(true).help("outputs to file specified, if not specified default to \"todos.txt\"."))
                            .arg(Arg::with_name("gitignore").short("g").long("gitignore").help("does not ignore gitignore paths if arg is passed").takes_value(false))
                            .arg(Arg::with_name("root").short("r").long("root").takes_value(true).help("specifies root directory."))
                            .arg(Arg::with_name("custom keyword").short("k").long("keyword").takes_value(true).help("Includes keyword specified as a word to search for.").multiple(false))
                            .get_matches();


    let outputfile = matches.value_of("outputfile").unwrap_or("./todos.txt");
    let root = matches.value_of("root").unwrap_or(".").to_owned();

    let gitignore:Option<HashSet<String>> = if !matches.is_present("gitignore"){
        get_ignorables()
    }
    else {
        None
    };

        
    
    let silent = matches.is_present("silent");
    let custom = matches.value_of("custom keyword").unwrap_or("");
    let mode = 1;
    
    let size_of_dir = if let Ok(sz) = get_size(std::path::Path::new(".")) {if matches.is_present("s") {0} else {sz}} else {0};

    let pb = ProgressBar::new(size_of_dir);
    pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:green}] {bytes}/{total_bytes} ({eta})\n {msg}").progress_chars("=> "));

    println!("{}","Searching for tasks...".bold());

    let (files_traversed, files_openned, todos): (i32,i32, LinkedList<Packet>) = iter_dir(root, &pb, &gitignore).expect(&format!("{}","Expecting Vector of data packets ".red().bold()));

    pb.finish_with_message(format!("✨ Done crawling {}/{} valid files in {} s",files_openned.to_string().green().bold(), files_traversed.to_string().green().bold(), pb.elapsed().as_secs_f32().to_string().green().bold()));

    return output_todos(mode, todos, Some(outputfile));

    
}
