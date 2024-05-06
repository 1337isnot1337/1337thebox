use std::process::Command;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() <= 4 {
        panic!("\nFor this script, enter the root user, root pass, target ip, and file path to the escalate file (or other).\nExample: 1337thebox root qwerty123 10.10.132.177 /home/thebestuser/1337thebox/escalate.sh\n");
    }

    let user = &args[1];
    let pass = &args[2];
    let otherip = &args[3];
    let file_path = &args[4];

    println!("Script initializing...\n Target user is {}, {}'s password is {}, target IP is {}, and the path to the escalate file is {}.", user, user, pass, otherip, file_path);
    
    match fs::read_to_string(file_path) {
        Ok(escalateFileContents) => {
            if escalateFileContents.trim().is_empty() {
                println!("The file is empty or contains only whitespace.");
            } else {
                
                run_ssh_commands(user, pass, otherip, &escalateFileContents);
            }
        },
        Err(e) => {
            println!("Failed to read the file: {}", e);
        }
    }
}

fn run_ssh_commands(user: &str, pass: &str, otherip: &str, escalateFileContents: &str) {
    if escalateFileContents.trim().is_empty() {
        println!("The escalate file is empty or contains only whitespace.");
        return;
    }

    let ssh_command = format!("sshpass -p '{}' scp escalate.sh {}@{}:~/ && sshpass -p '{}' ssh {}@{} 'chmod +x ~/escalate.sh && nohup sudo ~/escalate.sh >/dev/null 2>&1 &'",
                              pass, user, otherip, pass, user, otherip);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&ssh_command)
        .output()
        .expect("Failed to execute SSH command");

    if output.status.success() {
        println!("SSH command executed successfully!");
    } else {
        println!("SSH command failed to execute:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
