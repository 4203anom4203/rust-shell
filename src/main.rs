use std::io;

fn main() {
    loop { 
        println!("rsh> ");
        if let Err(e) = io::Write::flush(&mut io::stdout()) /* if this shit has a problem, throw an error */ {
            eprintln!("Error flushing stdout: {}", e);
            continue; //reset loop
        }
        

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading input: {}", e);
            continue;
        }

        //trim up the enter or \n character
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        
    }
}
