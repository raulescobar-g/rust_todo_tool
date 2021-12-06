use std::fs::*;
use std::io::{BufReader, BufRead, Error};
use std::fs;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

fn get_suspects() -> Result<Vec<String>, Error> {
    let mut suspects = vec![];
    let file = fs::read_to_string("./todo_suspects.yml")?;
    let mut s = String::new();

    for cha in file.chars(){
        if cha == '\n' {
            suspects.push(s);
            s = String::new();
        }
        else {
            s.push_str(&cha.to_string());
        }
    }
    if s.len() > 0 {suspects.push(s);}
    Ok(suspects)
}

//TODO: finish this part

fn iter_dir(path: String, suspects: &Vec<String>) -> Result<Vec<String>,Error> {
    let files = fs::read_dir(&path)?;
    let mut todos:Vec<String> = Vec::new();

    for file in files {
        if !file.as_ref().expect("File error in directory loop").path().is_dir() {
            match suspects.iter().find(|&x| x == file.as_ref().unwrap().path().into_os_string().into_string().as_ref().expect("Error looking for suspects")){
                None => continue,
                Some(file) => {
                    let data = File::open(file).expect("Expected file");
                    let buff = BufReader::new(data);

                    for line in buff.lines() {
                        
                        if line.as_ref().expect("Error reading line").to_uppercase().starts_with("//TODO") {
                            todos.push(line.as_ref().expect("line").replace("//TODO:","").trim().to_string());
                        }
                    }  
                    
                },
            }         
        }
        else {
            todos.append(&mut iter_dir(file.as_ref().unwrap().path().into_os_string().into_string().unwrap(), suspects).expect("Rercursion part"));
        }
    }
    return Ok(todos);
}
 
//TODO: readme
fn main() -> Result<(), Error>{
    let sus = get_suspects().expect("Error reading yml file.");
    let todos: Vec<String> = iter_dir(".".to_string(),&sus).expect("Error finding files.");
    File::create("./TodoPolice.txt").expect("Something went wrong openning file.");
    let mut tables = Table::new();
    tables.add_row(row!["TODO",bFg->"STATUS"]);
    for task in todos.iter() {
        tables.add_row(row![*task,bFg->" - "]);
    }
    fs::write("./TodoPolice.txt", &tables.to_string() ).expect("Error writing to file.");
    //tables.printstd();
    Ok(())
}
