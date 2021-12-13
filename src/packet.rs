


pub struct Packet {
    pub task: String,
    pub path : String,
    pub line_n : i32,
}

impl Packet {
    pub fn new(task: String, path: String,line_n: i32) -> Self {
        Packet {
            task:task, 
            path: path,
            line_n: line_n,
        }
    }

    
}