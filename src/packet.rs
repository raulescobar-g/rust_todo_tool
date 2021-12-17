use std::fs::{self,*};
use std::path::{Path};
use std::collections::{LinkedList, HashSet};
use std::io::{BufReader,BufRead,Error,ErrorKind};
use colored::*;
use indicatif::{ProgressBar};
use comment_parser::{self, SyntaxRule, get_syntax_from_path,CommentParser};
use std::fmt;
use prettytable::{Table};


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
}

impl Packet {
    pub fn new(task: String, path: String,urgency: Urgency) -> Self {
        Packet {
            task,
            path,
            urgency,
        }
    }
}

pub fn output_todos(mode: i32, todos: LinkedList<Packet>, outputfile: Option<&str>) -> Result<(), Error> {

    let mut tables = Table::new();
    tables.add_row(row![bFg->"TYPE",bFb->"TASK",bFw->"LOCATION"]);
    for task in todos.iter() {
        tables.add_row(row![ bFw->(*task).urgency, bFb->(*task).task,bFw->(*task).path]);
    }

    if let Some(filename) = outputfile {
        File::create(filename).expect(&format!("{} {}", "Something went wrong creating output file -> ".red().bold(),filename));
        fs::write(filename, &tables.to_string() ).expect(&format!("{} {}","Error writing to file -> ".red().bold(),filename));
    }
    
    if mode == 1 {tables.printstd();}

    Ok(())
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


pub fn iter_dir(path: String, pb: &ProgressBar, gitignore: &Option<HashSet<String>>) -> Result<(i32,i32, LinkedList<Packet>), Error> {
    let files = if let Ok(open_file) = fs::read_dir(&path) {open_file} else {return Err(Error::new(ErrorKind::Other, format!("{} {}", "Could not read the directory:".red().bold(),path)));};

    let mut todos:LinkedList<Packet> = LinkedList::new();
    let mut filecount = 0;
    let mut fileopenned = 0;

    for file in files {                                                     
        if let Ok(ref this_file) = file {                                               
            if let Some(gitfiles) = gitignore {
                if gitfiles.contains(&this_file.path().into_os_string().into_string().unwrap()) {
                    continue;
                }
            }
            if !this_file.path().is_dir() {                                                      
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
                if let Ok((numc,numo,mut packs)) = iter_dir(this_file.path().into_os_string().into_string().unwrap(),&pb, gitignore)  {
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



pub fn get_ignorables() -> Option<HashSet<String>> {
    let file = if let Ok(_file) = File::open(".gitignore"){
        _file
    }
    else {
        return None;
    };
    let reader = BufReader::new(file);
    let mut ignorables = HashSet::new();

    for line in reader.lines() {
        if let Ok(_line) = line {ignorables.insert(_line);}
    }
    println!("{:?}",ignorables);
    return Some(ignorables);
}