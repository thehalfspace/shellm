
struct App {
    input: String,
}

impl App {
    fn new() -> Self {
        Self { input: String::new() }
    }
    
    // Method to read user input from the command line and set it in `input` field.
    fn ask(&mut self) -> io::Result<()> {
        print!("Enter your command (type 'exit' to quit): ");
        
        self.input.clear();
        
        io::stdin().read_line(&mut self.input)?;
        
        Ok(())
    }
    
    // Method to execute the input as a bash command and print its output.
    fn run(&self) -> io::Result<()> {
        if let Ok(command) = Command::spawn(self.input.trim().to_owned()) {
            let output = command.output()?;
            
            print!("{}", String::from_utf8_lossy(&output.stdout));
            
            if !output.stderr.is_empty() {
                eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        } else {
            println!("Invalid command");
        }
        
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut app = App::new();
    
    loop {
        app.ask()?;
        
        if app.input == "exit" {
            break;
        } else {
            app.run()?;
        }
    }
    
    Ok(())
}
