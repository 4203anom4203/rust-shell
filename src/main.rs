use std::io;

fn main() {
    loop { 
        println!("rsh> ");
        if let Err(e) = io::Write::flush(&mut io::stdout()) {
            eprintln!("Error flushing stdout: {}", e);
            continue;
        }
        
    }
}
