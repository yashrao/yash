use std::io;
use std::io::Write;
use std::ffi::CString;
use nix::unistd::{fork, ForkResult, execvp};

fn main() {
    loop {
        let command = prompt_user();
        run_command(command);
    }
}


fn prompt_user() -> String {
    print!("YaSh-> ");
    io::stdout().flush()
        .expect("Failed to flush output");
    let mut command = String::new();
    io::stdin().read_line(&mut command)
        .expect("Failed to read line");
    command = command.trim().to_string();
    return command;
}

fn run_command(command: String) {
    let command_Cstr = CString::new((command).as_bytes()).unwrap();
    let mut child = false;
    let pid: i32 = -1;
    match fork() {
        Ok(ForkResult::Parent {..}) => child = false, 
        Ok(ForkResult::Child) => child = true,
        Err(_) => println!("Failed to run command"),
    }

    if child == true {
        let err = execvp(&command_Cstr, &[CString::new("echo").unwrap()]);
        match err {
            Err(e) => println!("Error: {}", e),
            _ => (),
        }
    }
}
