use std::collections::HashMap;

pub struct Suffix {
    lined : String,
    multilined : Option<Vec<String>, None>,
}

pub struct Packet {
    task: String,
    filename : String,
    path : String,
    suffix : Suffix,
    line_n : u8,
}

impl Packet {
    pub fn new(task: String, path: String) -> Self {
        let s = Suffix {lined: , multilined: };
        Packet {
            task:task, 
            filename: filename,
            path: path,
            suffix: s,
            line_n: line_n,
        }
    }
}