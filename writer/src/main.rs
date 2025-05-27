use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    
    for i in 0..5 {
        stdout.write_all(format!("Hello from writer: {}\n", i).as_bytes())?;
        stdout.flush()?; // Ensure message is sent immediately
        stderr.write_all(format!("[Writer] Sent: {}\n", i).as_bytes())?;
        stderr.flush()?;
        
        // For debugging stdout (parent console)
        thread::sleep(Duration::from_millis(500));
    }
    
    Ok(())
}