use std::io::{self, Write};

pub fn display(){
    io::stdout().flush();
    io::stdout().write(b"> ");
    io::stdout().flush();
}
