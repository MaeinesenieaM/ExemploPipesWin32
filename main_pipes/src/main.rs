use std::ffi::CString;
use std::mem::MaybeUninit;
use windows_sys::{
    Win32::System::{Threading::*, Pipes::*},
    Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
    Win32::Foundation::{HANDLE, CloseHandle, SetHandleInformation, HANDLE_FLAG_INHERIT, TRUE, FALSE},
    Win32::Security::SECURITY_ATTRIBUTES,
};
use std::{env, ptr};
use std::path::PathBuf;
//use windows_sys::Win32::Foundation::HANDLE_FLAGS;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut exe_path: PathBuf = env::current_exe()?; // Path to *this* parent executable
    exe_path.pop();
    // Ensure the path exists
    if !exe_path.exists() {
        eprintln!("Error: Child executable not found at {:?}", exe_path);
        eprintln!("Please build the child program first: `cd child_program && cargo build --release`");
        return Err(Box::from(format!("WHOOPS! Current path doesn't exist for some reason. {:?}", exe_path)));
    }

    let mut reader_path = PathBuf::from(&exe_path);
    let mut writer_path = PathBuf::from(&exe_path);
    reader_path.push("reader");
    writer_path.push("writer");
    println!("{:?}\n{:?}", reader_path, writer_path);

    let mut pipe_output: HANDLE = ptr::null_mut(); //read
    let mut pipe_input: HANDLE = ptr::null_mut();  //write

    let mut security_pipe = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: ptr::null_mut(),
        bInheritHandle: TRUE,
    };

    unsafe {
        CreatePipe(
            &mut pipe_output,
            &mut pipe_input,
            &mut security_pipe,
            0
        );

        let stdin_origem = GetStdHandle(STD_INPUT_HANDLE);
        let stdout_origem = GetStdHandle(STD_OUTPUT_HANDLE);
        
        //Aki nós trocamos a flag HANDLE_FLAG_INHERIT com outra HANDLE_FLAG_INHERIT.
        SetHandleInformation(pipe_output, HANDLE_FLAG_INHERIT, 0);
        
        let mut writer: STARTUPINFOA = MaybeUninit::zeroed().assume_init();
        writer.cb = size_of::<STARTUPINFOA>() as u32;
        writer.dwFlags = STARTF_USESTDHANDLES;
        writer.hStdOutput = pipe_input;
        writer.hStdError = stdout_origem;
        writer.hStdInput = stdin_origem;

        let mut command_line_writer: Vec<u8> = Vec::from(writer_path.into_os_string().as_encoded_bytes());
        command_line_writer.push(0);
        let mut writer_pi: PROCESS_INFORMATION = MaybeUninit::zeroed().assume_init();

        CreateProcessA(
            ptr::null(), // Use lpCommandLine for full path + args
            command_line_writer.as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            TRUE,
            CREATE_NO_WINDOW,
            ptr::null(),
            ptr::null(),
            &writer,
            &mut writer_pi,
        );
        
        SetHandleInformation(pipe_input, HANDLE_FLAG_INHERIT, 0);
        SetHandleInformation(pipe_output, HANDLE_FLAG_INHERIT , 1);

        let mut reader: STARTUPINFOA = MaybeUninit::zeroed().assume_init();
        reader.cb = size_of::<STARTUPINFOA>() as u32;
        reader.dwFlags = STARTF_USESTDHANDLES;
        reader.hStdOutput = stdout_origem;
        reader.hStdError = stdout_origem;
        reader.hStdInput = pipe_output;

        let mut command_line_reader: Vec<u8> = Vec::from(reader_path.into_os_string().as_encoded_bytes());
        command_line_reader.push(0);
        let mut reader_pi: PROCESS_INFORMATION = MaybeUninit::zeroed().assume_init();

        let success = CreateProcessA(
            ptr::null(), // Use lpCommandLine for full path + args
            command_line_reader.as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            TRUE,
            CREATE_NO_WINDOW,
            ptr::null(),
            ptr::null(),
            &reader,
            &mut reader_pi,
        );

        println!("{success}");
        
        SetHandleInformation(pipe_output, HANDLE_FLAG_INHERIT , 0);

        CloseHandle(pipe_output);
        CloseHandle(pipe_input);

        println!("Awaiting Writer to finish...");
        WaitForSingleObject(writer_pi.hProcess, INFINITE);
        println!("Awaiting Reader to finish...");
        WaitForSingleObject(reader_pi.hProcess, INFINITE);


        CloseHandle(writer_pi.hProcess);
        CloseHandle(writer_pi.hThread);
        CloseHandle(reader_pi.hProcess);
        CloseHandle(reader_pi.hThread);
    }

    println!("all done!");

    Ok(())
}