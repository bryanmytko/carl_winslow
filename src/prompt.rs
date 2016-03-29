use std::io::{self, Write};

pub fn display(){
    let _ = io::stdout().flush();
    let _ = io::stdout().write(b"> ");
    let _ = io::stdout().flush();
}

pub fn flush(){
    print!("\r");
}

pub fn output(string: &str) {
    flush();
    println!("{}", string);
    display();
}
