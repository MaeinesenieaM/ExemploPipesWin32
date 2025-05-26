use std::ffi::CString;
use std::mem::MaybeUninit;
use windows_sys::{
    Win32::System::{Threading::*, Pipes::*},
    Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
    Win32::Foundation::{HANDLE, CloseHandle, SetHandleInformation, HANDLE_FLAG_INHERIT, TRUE, FALSE},
    Win32::Security::SECURITY_ATTRIBUTES,
};
use std::{env, ptr};
//use windows_sys::Win32::Foundation::HANDLE_FLAGS;

static COUNTER: std::sync::RwLock<i32> = std::sync::RwLock::new(0);

extern "system" fn callback(_: PTP_CALLBACK_INSTANCE, _: *mut std::ffi::c_void, _: PTP_WORK) {
    let mut counter = COUNTER.write().unwrap();
    *counter += 1;
}

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut exe_path = env::current_exe()?; // Path to *this* parent executable
    exe_path.pop(); // Go up from `target/debug/parent_exe` to `target/debug/`

    #[cfg(debug_assertions)] // If running in debug mode for parent, assume child is also debug
    exe_path.push("child_program/target/debug/pipe_child.exe");
    #[cfg(not(debug_assertions))] // If running in release mode for parent, assume child is release
    exe_path.push("child_program/target/release/pipe_child.exe");

    // Ensure the path exists
    if !exe_path.exists() {
        eprintln!("Error: Child executable not found at {:?}", exe_path);
        eprintln!("Please build the child program first: `cd child_program && cargo build --release`");
        return Err(Box::from(format!("WHOOPS! Current path doesn't exist for some reason. {:?}", exe_path)));
    }

    let program_path_c_string = CString::new(exe_path.to_str().unwrap()).unwrap();

    let mut read_pipe: HANDLE = ptr::null_mut();
    let mut write_pipe: HANDLE = ptr::null_mut();

    let mut security_pipe = SECURITY_ATTRIBUTES {
        nLength: size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: ptr::null_mut(),
        bInheritHandle: FALSE,
    };

    unsafe {
        CreatePipe(
            &mut read_pipe,
            &mut write_pipe,
            &mut security_pipe,
            1
        );

        let stdin_origem = GetStdHandle(STD_INPUT_HANDLE);
        let stdout_origem = GetStdHandle(STD_OUTPUT_HANDLE);

        let mut writer: STARTUPINFOA = MaybeUninit::zeroed().assume_init();
        writer.cb = size_of::<STARTUPINFOA> as u32;
        writer.dwFlags = STARTF_USESTDHANDLES;
        writer.hStdOutput = write_pipe;
        writer.hStdError = stdout_origem;
        writer.hStdInput = stdin_origem;

        //Aki nós trocamos a flag HANDLE_FLAG_INHERIT com outra HANDLE_FLAG_INHERIT.
        SetHandleInformation(write_pipe, HANDLE_FLAG_INHERIT, 1);

        let mut command_line_writer: Vec<u8> = CString::new(
            format!("{:?} writer", program_path_c_string)
        ).unwrap().into_bytes();
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

        SetHandleInformation(write_pipe, HANDLE_FLAG_INHERIT, 0);
        SetHandleInformation(read_pipe, HANDLE_FLAG_INHERIT , 1);

        let mut reader: STARTUPINFOA = MaybeUninit::zeroed().assume_init();
        reader.cb = size_of::<STARTUPINFOA> as u32;
        reader.dwFlags = STARTF_USESTDHANDLES;
        reader.hStdOutput = stdout_origem;
        reader.hStdError = stdout_origem;
        reader.hStdInput = read_pipe;

        let mut command_line_reader: Vec<u8> = CString::new(
            format!("{:?} reader", program_path_c_string)
        ).unwrap().into_bytes();

        let mut reader_pi: PROCESS_INFORMATION = MaybeUninit::zeroed().assume_init();

        CreateProcessA(
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

    }

    println!("counter: {:p}", read_pipe);

    Ok(())
}
