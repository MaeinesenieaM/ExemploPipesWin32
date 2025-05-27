use std::io::{self, BufRead, BufReader};
use std::env;
use windows_sys::Win32::System::Console::GetStdHandle;
use windows_sys::Win32::System::Console::STD_INPUT_HANDLE;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    println!("stdin handle: {:?}", handle);
    
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock()); // Use BufReader for line-by-line reading
    let mut line = String::new();
    println!("[Reader] Waiting for input (line by line)...");

    loop {
        line.clear(); // Clear the buffer for the next line
        match reader.read_line(&mut line) { // Read one line at a time
            Ok(0) => { // 0 bytes read means EOF
                println!("[Reader] Received EOF (pipe closed).");
                break;
            },
            Ok(_) => {
                println!("[Reader] Received line: {}", line.trim());
            },
            Err(e) => {
                eprintln!("[Reader] Error reading: {}", e);
                break;
            }
        }
    }
    println!("[Reader] Finished reading.");
    Ok(())
}