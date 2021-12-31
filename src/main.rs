#[macro_use] extern crate prettytable;
mod packet;
use crate::packet::*;
use std::io::Error;
use std::collections::LinkedList;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Arg, App}; 



//TODO: setup github actions
//TODO: package app 
fn main() -> Result<(), Error>{
 
    let matches = App::new("todo police").version("0.1.0").author("Raul E. <raul3@microanalisis.com>").about("Todo comments aggregator.")
                            .arg(Arg::with_name("silent").short("s").long("silent").help("does not output to console (twice as fast)"))
                            .arg(Arg::with_name("outputfile").short("f").long("file").takes_value(true).help("outputs to file specified, if not specified default to \"todos.txt\"."))
                            .arg(Arg::with_name("gitignore").short("g").long("gitignore").help("does not ignore gitignore paths if arg is passed").takes_value(false))
                            .arg(Arg::with_name("root").short("r").long("root").takes_value(true).help("specifies root directory."))
                            .arg(Arg::with_name("custom keyword").short("k").long("keyword").takes_value(true).help("Includes keyword specified as a word to search for.").multiple(false))
                            .get_matches();


    let outputfile = matches.value_of("outputfile").unwrap_or("./todos.txt");
    let root = matches.value_of("root").unwrap_or(".").to_owned();

    let gitignore:Option<Vec<String>> = if !matches.is_present("gitignore") { get_ignorables() } else { None };

    let silent = matches.is_present("silent");
    let custom = matches.value_of("custom keyword").unwrap_or("");
    
    let pb: Option<ProgressBar> = if let Some(total_size) = get_size(&gitignore, &root){
        if !silent {
            let temp = ProgressBar::new(total_size);
            temp.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:green}] {bytes}/{total_bytes} ({eta})\n {msg}").progress_chars("=> "));
            println!("{}","Searching for tasks...".bold());
            Some(temp)
        }
        else {
            None
        }
    }
    else {
        None
    };
    //TODO: print perhaps like the path in a tree if requested in args
    let (files_traversed, files_openned, mut todos): (i32,i32, LinkedList<Packet>) = iter_dir(root, &pb, &gitignore, &custom.to_string()).expect(&format!("{}","Expecting Vector of data packets ".red().bold()));

    get_lines(&mut todos);

    if let Some(_pb) = pb {
        _pb.finish_with_message(format!("✨ Done crawling {}/{} valid files in {} s",files_openned.to_string().green().bold(), files_traversed.to_string().green().bold(), _pb.elapsed().as_secs_f32().to_string().green().bold()));
    }
    else {
        if !silent {println!("✨ Done");}
    }
    return if silent { Ok(()) } else { output_todos( todos, Some(outputfile), &custom.to_string()) }

    
}

//TODO: write some tests
