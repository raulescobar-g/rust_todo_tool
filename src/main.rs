mod packet;
#[macro_use] extern crate prettytable;
use crate::packet::*;
use std::fs::*;
use std::io::{Error, ErrorKind};
use std::fs;
use std::collections::LinkedList;
use std::time::{Instant};
use prettytable::{Table};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
 
//TODO: add a function that gets parameters from a config file
//TODO: add a function that gets files to ignore maybe from the gitignore
//TODO: finish integrating the packet class still updating class 
//TODO: maybe pring everything in like a tree pattern that mirrors the project structure

//TODO: package app somehow
//TODO: deploy to homebrew
//TODO: cache results on a file but print the actual stuff on the terminal

fn main() -> Result<(), Error>{

    //TODO:get config options 
    // print to terminal or file
    // gitignore file or similar
    // custom urgency type???

    let outputfile = "./TodoPolice.txt";
    
    let size_of_dir = if let Ok(sz) = get_size(std::path::Path::new(".")){
        sz
    }
    else {
        println!("{}", "Could not get size of directory".red().bold());
        1000000
    };

    let pb = ProgressBar::new(size_of_dir);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:green}] {bytes}/{total_bytes} ({eta})\n {msg}")
        .progress_chars("=> "));

    println!("{}","Searching for todos...".bold());

    let (files_traversed,files_openned, todos): (i32,i32, LinkedList<Packet>) = iter_dir(".".to_string(), &pb).expect(&format!("{}","Expecting Vector of data packets: ".red()));

    pb.finish_with_message(format!("✨ Done crawling {}/{} valid files in {} s",files_openned.to_string().green().bold(), files_traversed.to_string().green().bold(), pb.elapsed().as_secs_f32().to_string().green().bold()));

    File::create(outputfile).expect(&format!("{} {}", "Something went creating output file: ".red().bold(),outputfile));
    
    let mut tables = Table::new();
    tables.add_row(row![bFb->"TODO",bFw->"PATH",bFg->"LINE#"]);
    for task in todos.iter() {
        tables.add_row(row![bFb->(*task).task,bFw->(*task).path ,bFg->(*task).line_n]);
    }
    fs::write(outputfile, &tables.to_string() ).expect(&format!("{} {}","Error writing to file:".red().bold(),outputfile));
    //tables.printstd();


    Ok(())
}
