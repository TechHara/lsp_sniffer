use std::fs::File;
use std::io::Result;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    // Get the command to run from the first argument
    let args: Vec<_> = std::env::args().skip(1).collect();
    let binary_file = std::env::current_exe()?;
    let binary_dir = binary_file.parent().unwrap();
    let command = binary_dir.join(Path::new("lsp_server"));
    let mut msg_from_server = File::create(binary_dir.join("msg_from_server.txt"))?;
    let mut msg_from_client = File::create(binary_dir.join("msg_from_client.txt"))?;

    // Build the command with piped stdin and stdout
    let mut process = Command::new(command)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    // Get handles to the process's stdin and stdout pipes
    let mut stdin = process.stdin.take().expect("Failed to get stdin");
    let mut stdout = process.stdout.take().expect("Failed to get stdout");

    // Thread to read from stdin and write to process's stdin
    let stdin_thread = std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        while let Ok(n) = io::stdin().read(&mut buffer) {
            if n == 0 {
                break;
            }
            if stdin.write_all(&buffer[..n]).is_err() || stdin.flush().is_err() {
                break;
            }
            if msg_from_client.write_all(&buffer[..n]).is_err() {
                break;
            }
        }
    });

    // Read from the process's stdout and write to our own stdout
    let mut buffer = [0; 1024];
    while let Ok(n) = stdout.read(&mut buffer) {
        if n == 0 {
            break;
        }
        io::stdout().write_all(&buffer[..n])?;
        io::stdout().flush()?;
        msg_from_server.write_all(&buffer[..n])?;
    }

    // Wait for the child process to finish
    process.wait().expect("Failed to wait for process");

    // Wait for the stdin thread to finish
    stdin_thread.join().expect("Failed to join stdin thread");

    Ok(())
}
