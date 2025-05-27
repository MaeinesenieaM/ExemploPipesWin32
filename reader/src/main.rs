use std::io::{self, BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    //Guarda as entradas e saídas do processo.
    let stdin = io::stdin();
    
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    
    //Prende o stdin para que nenhum outro processo, atrapalhe.
    let mut reader = BufReader::new(stdin.lock());
    //Cria uma String vazia para guardar os valores.
    let mut line = String::new();
    
    stderr.write("[Reader] Waiting for input (line by line)...\n".as_bytes())?;
    stderr.flush()?;

    loop {
        line.clear(); // Limpa o buffer da linha.
        match reader.read_line(&mut line) { // read_line ler dados até chegar em '\n'.
            Ok(0) => { //O número 0 significa 
                stdout.write("[Reader] Received EOF.\n".as_bytes())?;
                stdout.flush()?;
                break;
            },
            Ok(_) => {
                stdout.write(format!("[Reader] Received line: {}\n", line.trim()).as_bytes())?;
                stdout.flush()?;
            },
            Err(e) => {
                stderr.write(format!("[Reader] Error reading: {}\n", e).as_bytes())?;
                stderr.flush()?;
                break;
            }
        }
    }
    
    stderr.write("[Reader] Finished reading..\n".as_bytes())?;
    stderr.flush()?;
    Ok(())
}