use std::io::{self, Write};

pub fn display(){
    io::stdout().flush();
    io::stdout().write(b"> ");
    io::stdout().flush();
}

pub fn flush(){
    print!("\r");
}

pub fn output(string: &str) {
    flush();
    println!("{}", string);
    display();
}
