use std::mem::MaybeUninit;
use windows_sys::{
    Win32::System::{Threading::*, Pipes::*},
    Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
    Win32::Foundation::{HANDLE, CloseHandle, SetHandleInformation, HANDLE_FLAG_INHERIT, TRUE},
};
use std::{env, ptr};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    //Registra o caminho atual do arquivo.
    let mut exe_path: PathBuf = env::current_exe()?;
    exe_path.pop(); //Simplesmente transforma na pasta que o processo esta rodando.

    //Registra caminhos para os outros processos.
    let mut reader_path = PathBuf::from(&exe_path);
    let mut writer_path = PathBuf::from(&exe_path);
    reader_path.push("reader");
    writer_path.push("writer");

    //ptr::null_mut se equivale a void* NULL em C.
    let mut pipe_output: HANDLE = ptr::null_mut(); //read
    let mut pipe_input: HANDLE = ptr::null_mut();  //write

    unsafe {
        let sucesso = CreatePipe(
            &mut pipe_output,
            &mut pipe_input,
            ptr::null(),
            4096
        );
        if sucesso != 1 {
            panic!("Erro ao criar tubo!")
        };

        let stdin_origem = GetStdHandle(STD_INPUT_HANDLE);
        let stdout_origem = GetStdHandle(STD_OUTPUT_HANDLE);
        
        //Cria um STARTUPINFOA vazio.
        let mut writer: STARTUPINFOA = MaybeUninit::zeroed().assume_init();
        writer.cb = size_of::<STARTUPINFOA>() as u32;
        writer.dwFlags = STARTF_USESTDHANDLES;
        writer.hStdOutput = pipe_input; //Registra a entrada do Pipe na saida do mensageiro.
        writer.hStdError = stdout_origem;   
        writer.hStdInput = stdin_origem;

        let mut reader: STARTUPINFOA = MaybeUninit::zeroed().assume_init();
        reader.cb = size_of::<STARTUPINFOA>() as u32;
        reader.dwFlags = STARTF_USESTDHANDLES;
        reader.hStdOutput = stdout_origem;
        reader.hStdError = stdout_origem;
        reader.hStdInput = pipe_output; //Registra a saida do Pipe na entrada do leitor.
        
        //Conversão para UTF-8.
        let mut writer_path_bytes: Vec<u8> = Vec::from(writer_path.into_os_string().as_encoded_bytes());
        writer_path_bytes.push(0);
        let mut reader_path_bytes: Vec<u8> = Vec::from(reader_path.into_os_string().as_encoded_bytes());
        reader_path_bytes.push(0);

        //Cria informações vazias para serem preenchidas por CreateProcess().
        let mut writer_pi: PROCESS_INFORMATION = MaybeUninit::zeroed().assume_init();
        let mut reader_pi: PROCESS_INFORMATION = MaybeUninit::zeroed().assume_init();
        
        //Precisamos desativar e ativar o herdar mento das entradas/saídas para garantir
        //que as certas sejam escolhidas.
        SetHandleInformation(pipe_input, HANDLE_FLAG_INHERIT, 1);
        SetHandleInformation(pipe_output, HANDLE_FLAG_INHERIT, 0);

        //Cria o mensageiro.
        CreateProcessA(
            ptr::null(),
            writer_path_bytes.as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            TRUE, //Garante que as entradas e saídas do Tubo sejam aceitas.
            0,
            ptr::null(),
            ptr::null(),
            &writer,
            &mut writer_pi,
        );
        
        SetHandleInformation(pipe_input, HANDLE_FLAG_INHERIT, 0);
        SetHandleInformation(pipe_output, HANDLE_FLAG_INHERIT , 1);

        //Cria o leitor.
        CreateProcessA(
            ptr::null(),
            reader_path_bytes.as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            TRUE,
            0,
            ptr::null(),
            ptr::null(),
            &reader,
            &mut reader_pi,
        );
        
        SetHandleInformation(pipe_output, HANDLE_FLAG_INHERIT , 0);

        //Após a criação dos processos fechamos os acessos para as entradas nesse processo
        //para que outros processos possam utilizá-lo.
        CloseHandle(pipe_output);
        CloseHandle(pipe_input);

        println!("[CONSTRUCTOR] Esperando que o Leitor termine...");
        WaitForSingleObject(writer_pi.hProcess, INFINITE);
        println!("[CONSTRUCTOR] Esperando que o Mensageiro termine...");
        WaitForSingleObject(reader_pi.hProcess, INFINITE);

        //Fecha os ponteiros para os processos.
        CloseHandle(writer_pi.hProcess);
        CloseHandle(writer_pi.hThread);
        CloseHandle(reader_pi.hProcess);
        CloseHandle(reader_pi.hThread);
    }

    println!("[CONSTRUCTOR] Pronto!");

    Ok(())
}