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

        let mut parts = input.split_whitespace(); //returns an iterator
        let command = parts.next().expect("Should contain a string."); //command is the first slice of the input string, the first word
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => {
                break;
            },
            "cd" => {
                let n_dir = args.get(0).copied();

                if let Some(path) = n_dir {
                    if let Err(e) = std::env::set_current_dir(path) {
                        eprintln!("cd: {}", e);
                    }
                }
                continue;
            }
            
            _ => {
                
            }
        }
    }
}
