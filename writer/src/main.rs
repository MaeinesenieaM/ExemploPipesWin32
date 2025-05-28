use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    
    for i in 0..5 {
        stdout.write_all(format!("Ola vizinho!: {}\n", i).as_bytes())?;
        stdout.flush()?; // Ensure message is sent immediately
        stderr.write_all(format!("[Writer] Enviou seu Ola: {}\n", i).as_bytes())?;
        stderr.flush()?;
        
        thread::sleep(Duration::from_millis(500));
    }
    
    Ok(())
}