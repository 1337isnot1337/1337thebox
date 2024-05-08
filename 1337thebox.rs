//import relevant crates and stuff
use std::process::Command;
use std::env;
use std::fs;


fn main() {
    //get the flags
    let args: Vec<String> = env::args().collect();
    //if no flags
    if args.len() <= 1 {
        panic!("\nFor this script, enter the root user, root pass, target ip, and file name (NOT FULL PATH!). Optionally, run the script with the previous condtions, but add kill to the end to kill the process.\nExample: ./1337thebox root qwerty123 10.10.132.177 escalate.sh\n");
    }
    //setting vars based off of flags
    let user = &args[1];
    let pass = &args[2];
    let target_ip = &args[3];
    let file_name = &args[4];
    //handle kill request
    
    
    let kill_status = if args.len() == 6 && args[5] == "kill" {
        true
    } else {
        false
    };
    
    println!("Script initializing...\n Target user is {}, {}'s password is {}, target IP is {}, and the path to the escalate file is {}.", user, user, pass, target_ip, file_name);
    //read file, handle any error
    match fs::read_to_string(file_name) {
        //Read successfully:
        Ok(file_contents) => {
            if file_contents.trim().is_empty() {
                println!("The file is empty or contains only whitespace, please give a different file.");
            } else {
                
                run_ssh_commands(user, pass, target_ip, file_name, kill_status);
            }
        },
        //Read unsuccessfully
        Err(e) => {
            println!("Failed to read the file. Error is as follows: {}", e);
        }
    }
}

//ssh command function:
fn run_ssh_commands(user: &str, pass: &str, target_ip: &str, file_name: &str, kill_status: bool) {
    //command for ssh
    let ssh_command = if kill_status {
        format!("sshpass -p '{}' ssh {}@{} 'pkill -x {}'"
        , pass, user, target_ip, file_name)
    } else {
        format!("sshpass -p '{}' scp {} {}@{}:~/ && sshpass -p '{}' ssh -f {}@{} 'chmod +x ~/{} && nohup ~/{} {} >/dev/null 2>&1 &'"
        , pass, file_name, user, target_ip, pass, user, target_ip, file_name, file_name, pass)
    };
	println!("{}", ssh_command);    
    //command running settings
    let mut output = Command::new("sh")
        .arg("-c")
        .arg(&ssh_command)
        .spawn()
        .expect("Failed to execute SSH, not my problem :3");
    //ssh status reciept
    let result = output.wait().expect("Failed to wait for command");
    println!("{}", result)

}
