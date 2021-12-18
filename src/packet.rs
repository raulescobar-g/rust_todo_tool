use std::fs::{self,*};
use std::collections::{LinkedList};
use std::io::{BufReader,BufRead,Error,ErrorKind};
use colored::*;
use indicatif::{ProgressBar};
use comment_parser::{self, SyntaxRule, get_syntax_from_path,CommentParser};
use std::fmt;
use prettytable::{Table};
use chrono::prelude::*;

#[derive(PartialEq)]
pub enum Urgency {
    TODO,
    FIXME,
    HACK,
    XXX,
    CUSTOM,
}

impl fmt::Display for Urgency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Urgency::TODO => write!(f, "TODO"),
           Urgency::FIXME => write!(f, "FIXME"),
           Urgency::HACK => write!(f, "HACK"),
           Urgency::XXX => write!(f, "XXX"),
           _ => write!(f, ""),
       }
    }
}
pub struct Packet {
    pub task: String,
    pub path : String,
    pub urgency: Urgency,
    pub line : i32,
}

impl Packet {
    pub fn new(task: String, path: String,urgency: Urgency) -> Self {
        Packet {
            task,
            path,
            urgency,
            line : 0,
        }
    }
}

pub fn output_todos(todos: LinkedList<Packet>, outputfile: Option<&str>, custom: &String) -> Result<(), Error> {

    let mut tables = Table::new();
    tables.add_row(row![bFg->"Type",bFw->"Task",bFm->"Path",bFr->"Line"]);
    for task in todos.iter() {
        if (*task).urgency == Urgency::CUSTOM {
            tables.add_row(row![bFg->custom , bFw->(*task).task,bFm->(*task).path,bFr->(*task).line]);
        }
        else {
            tables.add_row(row![bFg->(*task).urgency , bFw->(*task).task,bFm->(*task).path,bFr->(*task).line]);
        }
        
    }

    if let Some(filename) = outputfile {
        File::create(filename).expect(&format!("{} {}", "Something went wrong creating output file -> ".red().bold(),filename));
        let right_now = Local::now();
        fs::write(filename, format!("Updated: {}\n{}",right_now.to_string(), &tables.to_string())).expect(&format!("{} {}","Error writing to file -> ".red().bold(),filename));
    }
    
    tables.printstd();

    Ok(())
}

pub fn get_size(gitignore: &Option<Vec<String>>, path: &String) -> Option<u64> {
    let mut result = 0;
    let files = if let Ok(open_file) = fs::read_dir(&path) {open_file} else {return None;};

    for entry in files {
        if let Ok(_entry) = entry{
            let ignorable = should_ignore(gitignore, &_entry);
            if !_entry.path().is_dir() && !ignorable {
                result += if let Ok(meta) = _entry.metadata() {
                    meta.len()
                }
                else {
                    println!("Did not read file: {:?}",_entry);
                    0
                }
            } 
            else if !ignorable {
                result += if let Some(res) = get_size(gitignore,&_entry.path().into_os_string().into_string().unwrap()){res} else {0}
            }
        }
    }
    
    Some(result)
}

fn matcher(filename: &DirEntry, comment_rules: &[SyntaxRule], todos: &mut LinkedList<Packet>, custom : &String) -> Result<(),Error> {
    
    if let Ok(entire_file) = read_to_string(filename.path()){
        let parser = CommentParser::new(entire_file.as_str(), comment_rules);

        for comments in parser {
            if let Some(_urgency) = comments.text().to_string().split_once(":").map(|s| (s.0.trim(),s.1.trim())){
                let urgency = match _urgency.0 {
                    "TODO" => {Urgency::TODO},
                    "FIXME" => {Urgency::FIXME},
                    "HACK" => {Urgency::HACK},
                    "XXX" => {Urgency::XXX},
                    other => {if other == custom {Urgency::CUSTOM} else {continue;}},
                };

                let pack = Packet::new(_urgency.1.to_string(), filename.path().into_os_string().into_string().unwrap(), urgency);
                todos.push_back(pack);
            }
                
        }
            
        
    }            
    else {
        return Err(Error::new(ErrorKind::Other, "File is not utf-8, or could not read lines for some reason."));
    }

    

    return Ok(());
}

fn get_todos(filename: &DirEntry, custom : &String) -> Option<LinkedList<Packet>> {

    let pathy = filename.path();
    let rules = if let Ok(syntax_rules) = get_syntax_from_path(pathy){
        syntax_rules
    }
    else {
        return None;
    };

    let mut todos:LinkedList<Packet> = LinkedList::new();
    

    if let Ok(_) = matcher(filename, rules, &mut todos, custom){
        return Some(todos);
    }
    else {
        return None;
    }
}

fn should_ignore(gitignore: &Option<Vec<String>>, this_file: &DirEntry) -> bool{
    if let Some(gitfiles) = gitignore {
        for filename in gitfiles {
            if this_file.path().into_os_string().into_string().unwrap().ends_with(filename){
                return true;
            }
        }
    }
    return false;
}

pub fn iter_dir(path: String, pb: &Option<ProgressBar>, gitignore: &Option<Vec<String>>, custom : &String) -> Result<(i32,i32, LinkedList<Packet>), Error> {
    let files = if let Ok(open_file) = fs::read_dir(&path) {open_file} else {return Err(Error::new(ErrorKind::Other, format!("{} {}", "Could not read the directory:".red().bold(),path)));};

    let mut todos:LinkedList<Packet> = LinkedList::new();
    let mut filecount = 0;
    let mut fileopenned = 0;
    let mut ignorable;

    for file in files {                                                     
        if let Ok(ref this_file) = file { 
                                                       
            ignorable = should_ignore(gitignore,this_file);

            if !this_file.path().is_dir() && !ignorable {                                                      
                if let Some(_pb) = pb {_pb.inc(this_file.metadata().expect("Cant open this files metadata: ").len());}
                filecount += 1;
                if let Some(mut b) = get_todos(this_file, custom) {
                    fileopenned += 1;
                    todos.append(&mut b);
                }
                else {
                    continue;
                }
            }
            else if !ignorable {
                if let Ok((numc,numo,mut packs)) = iter_dir(this_file.path().into_os_string().into_string().unwrap(),&pb, gitignore, custom)  {
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

pub fn get_ignorables() -> Option<Vec<String>> {
    let file = if let Ok(_file) = File::open(".gitignore"){
        _file
    }
    else {
        return None;
    };
    let reader = BufReader::new(file);
    let mut ignorables:Vec<String> = Vec::new();
    ignorables.push(".git".to_string());
    for line in reader.lines() {
        if let Ok(_line) = line {ignorables.push(_line);}
    }
    return Some(ignorables);
}

pub fn get_lines(todos: &mut LinkedList<Packet>) {
    for item in todos.iter_mut() {
        if let Ok(entire_file) = fs::read_to_string(&item.path) {
            let mut line_num = 1;
            for line in entire_file.lines() {
                if line.contains(&item.task){
                    item.line = line_num;
                    break;
                }
                line_num += 1;
            }
        }
    }
}