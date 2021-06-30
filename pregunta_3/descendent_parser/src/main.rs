mod desc_parser;
mod test_suite;
use std::io;
use std::io::Write;

// para ver mas casos, ir a test_suite.rs
fn main() {

    let mut parser = desc_parser::Parser::new();

    // Command buffer: store user input in this line
    let mut line = String::new();
    println!("Escribe la expresión a parsear");
    print!(">> "); // print prompt


    // flush so the print! doesn't mess up the execution order with read_line
    io::stdout().flush().expect("Couldn't flush stdout"); 

    // Read a single line
    if let Err(_) = io::stdin().read_line(&mut line) { panic!("Error leyendo input D:") }

    match parser.parse_string(line.as_str()) {
        None => println!("Esa frase no pertenece al lenguaje"),
        Some(t) => println!("El tipo de esa frase es: \n\t{}", t)
    }
}
