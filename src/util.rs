use std::io::Read;

pub fn make_indent(indent: u32) -> String {
    let mut indents = String::new();
    for _ in 0..indent {
        indents.push(' ');
    }
    indents
}


pub fn get_input() -> String {
    let file_name = std::env::args().nth(1);

    match file_name {
        None => {
            // read from stdin
            let mut buffer = String::new();
            let stdin = std::io::stdin();
            let mut lock = stdin.lock();
            lock.read_to_string(&mut buffer).unwrap();
            buffer
        }
        Some(file_name) => {
            // read from file
            std::fs::read_to_string(&file_name).unwrap()
        }
    }
}

