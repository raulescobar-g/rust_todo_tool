use std::fs::*;
use std::fs;
use std::path::Path;
use std::collections::LinkedList;
use std::{thread, time::Duration};
use std::io::{BufReader, BufRead, Error,ErrorKind};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use comment_parser::{self, SyntaxRule, get_syntax_from_path,CommentParser};


pub enum Urgency {
    TODO,
    FIXME,
    HACK,
    XXX,
}
pub struct Packet {
    pub task: String,
    pub path : String,
    pub line_n : i32,
    pub urgency: Urgency,
}

impl Packet {
    pub fn new(task: String, path: String,line_n: i32, urgency: Urgency) -> Self {
        Packet {
            task,
            path,
            line_n,
            urgency,
        }
    }

    
}

pub fn get_size<P>(path: P) -> std::io::Result<u64> where P: AsRef<Path>,{
    let mut result = 0;

    if path.as_ref().is_dir() {
        for entry in read_dir(&path)? {
            let _path = entry?.path();
            if _path.is_file() {
                result += _path.metadata()?.len();
            } else {
                result += get_size(_path)?;
            }
        }
    } else {
        result = path.as_ref().metadata()?.len();
    }
    Ok(result)
}


fn matcher(filename: &DirEntry, comment_rules: &[SyntaxRule], todos: &mut LinkedList<Packet>) -> Result<(),Error> {
    
    if let Ok(entire_file) = read_to_string(filename.path()){
        let parser = CommentParser::new(entire_file.as_str(), comment_rules);

        for comments in parser {
            if let Some(_urgency) = comments.text().to_string().split_once(":").map(|s| (s.0.trim(),s.1.trim())){
                let urgency = match _urgency.0 {
                    "TODO" => {Urgency::TODO},
                    "FIXME" => {Urgency::FIXME},
                    "HACK" => {Urgency::HACK},
                    "XXX" => {Urgency::XXX},
                    _ => {continue;},
                };

                let pack = Packet::new(_urgency.1.to_string(), filename.path().into_os_string().into_string().unwrap(), 0, urgency);
                todos.push_back(pack);
            }
                
        }
            
        
    }            
    else {
        return Err(Error::new(ErrorKind::Other, "File is not utf-8, or could not read lines for some reason."));
    }

    

    return Ok(());
}

fn get_todos(filename: &DirEntry) -> Option<LinkedList<Packet>> {

    let pathy = filename.path();
    let rules = if let Ok(syntax_rules) = get_syntax_from_path(pathy){
        syntax_rules
    }
    else {
        return None;
    };

    let mut todos:LinkedList<Packet> = LinkedList::new();
    

    if let Ok(_) = matcher(filename, rules, &mut todos){
        return Some(todos);
    }
    else {
        return None;
    }
}


pub fn iter_dir(path: String, pb: &ProgressBar) -> Result<(i32,i32, LinkedList<Packet>), Error> {
    let files = if let Ok(open_file) = fs::read_dir(&path) {open_file} else {return Err(Error::new(ErrorKind::Other, format!("{} {}", "Could not read the directory:".red().bold(),path)));};

    let mut todos:LinkedList<Packet> = LinkedList::new();
    let mut filecount = 0;
    let mut fileopenned = 0;
    

    for file in files {                                                     // for all entries in directory
        if let Ok(ref this_file) = file {                                               // if this file is something and we have  permission to read it
            if !this_file.path().is_dir() {                                                      // if this is a file then we try to get todos, if we fail then we continue the loop and increase filecount
                pb.inc(this_file.metadata().expect("Cant open this files metadata: ").len());
                filecount += 1;
                if let Some(mut b) = get_todos(this_file) {
                    fileopenned += 1;
                    todos.append(&mut b);
                }
                else {
                    continue;
                }
            }
            else {
                if let Ok((numc,numo,mut packs)) = iter_dir(this_file.path().into_os_string().into_string().unwrap(),&pb) {
                    todos.append(&mut packs);
                    filecount += numc;
                    fileopenned += numo;
                }
                else {
                    println!("{}{}","Could not open file/directory: ".red().bold(), this_file.file_name().as_os_str().to_str().unwrap());
                    continue;
                };
                
            }
        }
        else {
            return Err(Error::new(ErrorKind::Other, "No directory entries."));
        }
    }
    return Ok((filecount,fileopenned,todos));
}