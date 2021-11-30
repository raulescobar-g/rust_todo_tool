use std::fs;

fn get_suspects() -> Vec<String>{
    let mut suspects = vec![];
    let file = fs::read_to_string("./todo_suspects.yml").expect("No .yml file found");
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
    return suspects;
}

fn iter_dir(path: String, suspects: &Vec<String>) {
    let files = fs::read_dir(&path).unwrap();

    for file in files {

        if !file.as_ref().unwrap().path().is_dir() {
            if suspects.iter().find(|&x| x == file.as_ref().unwrap().path().into_os_string().into_string().as_ref().unwrap()) == None {
                continue;
            }
            let data = fs::read_to_string(file.as_ref().unwrap().path()).expect("Unable to read file");
            println!("{:?}",data)
        }
        else {
            iter_dir(file.as_ref().unwrap().path().into_os_string().into_string().unwrap(), suspects)
        }
        
    }
}

fn main() {
    let sussy = get_suspects();

    iter_dir(".".to_string(),&sussy)
}
