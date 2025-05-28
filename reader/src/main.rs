use std::io::{self, BufRead, BufReader, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Guarda as entradas e saídas do processo.
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    
    //Prende o stdin para que nenhum outro processo, atrapalhe.
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    
    stderr.write("[Reader] Esperando por entrada (linha por linha)...\n".as_bytes())?;
    stderr.flush()?;
    
    loop {
        line.clear(); // Limpa o buffer da linha.
        match reader.read_line(&mut line) { // read_line ler dados até chegar em '\n'.
            Ok(0) => {
                stdout.write("[Reader] Recebido o fim do arquivo. EOF\n".as_bytes())?;
                stdout.flush()?;
                break;
            },
            Ok(_) => {
                stderr.write(format!("[Reader] Linha recebida: {}\n", line.trim()).as_bytes())?;
                stderr.flush()?;
            },
            Err(e) => {
                stderr.write(format!("[Reader] Erro ocorrido ao ler: {}\n", e).as_bytes())?;
                stderr.flush()?;
                break;
            }
        }
    }
    
    stderr.write("[Reader] Terminando de ler..\n".as_bytes())?;
    stderr.flush()?;
    Ok(())
}