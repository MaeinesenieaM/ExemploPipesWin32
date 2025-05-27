use std::io::{self, Write};
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut stdout = io::stdout();
    for i in 0..5 {
        let message = format!("Hello from writer: {}\n", i);
        stdout.write_all(message.as_bytes())?;
        stdout.flush()?; // Ensure message is sent immediately
        //println!("[Writer] Sent: {}\n", message.trim()); // For debugging stdout (parent console)
        thread::sleep(Duration::from_millis(500));
    }
    
    Ok(())
}