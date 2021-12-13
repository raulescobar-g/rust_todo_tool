mod packet;
#[macro_use] extern crate prettytable;
use crate::packet::Packet;
use std::fs::*;
use std::io::{BufReader, BufRead, Error,ErrorKind};
use std::fs;
use std::time::{Instant};
use prettytable::{Table};
use colored::*;

fn iter_dir(path: String) ->Result<(i32, Vec<Packet>),Error> {
    let files = if let Ok(open_file) = fs::read_dir(&path) {open_file} else {return Err(Error::new(ErrorKind::Other, format!("Could not read the directory: {}",path)));};

    let mut todos:Vec<Packet> = Vec::new();
    let mut filecount = 0;

    for file in files {
        if let Ok(ref this_file) = file {
            if !this_file.path().is_dir() {
                filecount += 1;

                // panics when it gets here because it cannot open the file, make it optional to just skip this error
                let data = if let Ok(open_file) = File::open(this_file.path()){open_file} else {return Err(Error::new(ErrorKind::Other, format!("Could not open file: {}",this_file.path().as_os_str().to_str().unwrap())))};
                let buff = BufReader::new(data);
                let mut line_number = 0;

                let pathy = this_file.path();

                let comment_pattern: &str = if let Some(ext) = pathy.extension(){ 
                    let ans = match ext.to_os_string().to_str() {
                        Some("rs")| Some("cpp")| Some("js")| Some("h")|Some("java") | Some("ts") => {"//"},
                        Some("py") => {"#"},
                        None => {
                            filecount -= 1;
                            continue;
                        },
                        _ => {
                            filecount-= 1;
                            continue;
                        },
                    };
                    ans
                }
                else {
                    filecount -= 1;
                    continue;
                };

                for line in buff.lines() {
                    line_number +=1;
                    if let Ok(ref this_line) = line {                    
                        if this_line.to_uppercase().trim().starts_with(comment_pattern) {
                            let pack = Packet::new(this_line.trim().trim_start_matches( comment_pattern ).trim().to_string(), this_file.path().into_os_string().into_string().unwrap(), line_number);
                            todos.push(pack);
                        }
                    }
                    else {
                        filecount -= 1;
                        continue;
                    }

                }  
            }
            else {
                let (f, mut iter_res) = if let Ok((num,packs)) = iter_dir(this_file.path().into_os_string().into_string().unwrap()) {
                    (num, packs)
                }
                else {
                    println!("{}{}","Could not open file/directory: ".red().bold(),this_file.file_name().as_os_str().to_str().unwrap());
                    continue;
                };
                todos.append(&mut iter_res);
                filecount += f;
            }
        }
        else {
            return Err(Error::new(ErrorKind::Other, "No directory entries."));
        }
    }
    return Ok((filecount,todos));
}

//TODO: refactor iter_dir function its getting kinda big and should seperate concerns better
 
//TODO: add a function that gets parameters from a config file
//TODO: add a function that gets files to ignore maybe from the gitignore
//TODO: finish integrating the packet class still updating class 
//TODO: maybe pring everything in like a tree pattern that mirrors the project structure

//TODO: add comment patterns for multi types of code files => multiline codes and bodies of text
//TODO: add regex to handle different todo comment formats
//TODO: add FIXME, and HACK as like an urgency system, perhaps add this to the packet class as enum

//TODO: package app somehow
//TODO: deploy to homebrew
//TODO: cache results on a file but print the actual stuff on the terminal



fn main() -> Result<(), Error>{

    //get config options 

    //start timer and start file crawl

    let start = Instant::now();

    let (files_traversed, todos): (i32, Vec<Packet>) = iter_dir(".".to_string()).expect(&format!("{}","Expecting Vector of data packets: ".red()));

    let outputfile = "./TodoPolice.txt";
    File::create(outputfile).expect(&format!("{} {}", "Something went creating output file: ".red().bold(),outputfile));

    
    let mut tables = Table::new();
    tables.add_row(row![bFb->"TODO",bFw->"PATH",bFg->"LINE#"]);
    for task in todos.iter() {
        tables.add_row(row![bFb->(*task).task,bFw->(*task).path ,bFg->(*task).line_n]);
    }
    fs::write(outputfile, &tables.to_string() ).expect(&format!("{} {}","Error writing to file:".red().bold(),outputfile));
    tables.printstd();

    let duration = start.elapsed();
    //end of crawl and timer stop

    //print time taken for single threaded crawl
    println!("Files traversed: {} files",files_traversed.to_string().purple().magenta().bold());
    println!("Time elapsed:    {} ms",duration.as_millis().to_string().purple().magenta().bold());


    Ok(())
}
