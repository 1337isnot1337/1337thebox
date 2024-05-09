//import relevant crates and stuff
use std::process::Command;
use std::env;
use std::fs;

fn main() {
    //get the flags
    let args: Vec<String> = env::args().collect();
    //if no flags
    if args.len() <= 1 {
        panic!("\nFor this script, enter the root user, root pass, target ip, and file name (NOT FULL PATH!). Optionally, run the script with the previous condtions, but add kill to the end to kill the process.\nExample: ./1337thebox --user root --pass qwerty123 --target_ip 10.10.132.177 --file_name escalate.sh\n");
    }
    //setting init vars
        let mut user = "";
        let mut pass = "";
        let mut target_ip = "";
        let mut file_name = "";
        let mut kill_status = false;
    
    //iterating over args
    for i in 1..args.len() {
        match args[i].as_str() {
            "--user" => {
                if let Some(user_) = args.get(i + 1) {
                    user = user_;
                }
            }
            "--pass" => {
                if let Some(pass_) = args.get(i + 1) {
                    pass = pass_;
                }
            }
            "--target_ip" => {
                if let Some(target_ip_) = args.get(i + 1) {
                    target_ip = target_ip_;
                }
            }
            "--file_name" => {
                if let Some(file_name_) = args.get(i + 1) {
                    file_name = file_name_;
                }
            }
            "--pkill" => {
                kill_status = true;
            },
            &_ => {}
            
        }
    }
    
    println!("Script initializing...\n Target user is {}, {}'s password is {}, target IP is {}, and the path to the included file is {}.", user, user, pass, target_ip, file_name);
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
        format!("sshpass -p '{}' ssh {}@{} 'pkill -x {} && rm {}'"
        , pass, user, target_ip, file_name, file_name)
    } else {
        format!("sshpass -p '{}' scp {} {}@{}:~/ && sshpass -p '{}' ssh -f {}@{} 'chmod +x ~/{} && nohup ~/{} {} >/dev/null 2>&1 &'"
        , pass, file_name, user, target_ip, pass, user, target_ip, file_name, file_name, pass)
    };
    let check_command = format!("sshpass -p '{}' ssh {}@{} 'pgrep -f {}'", pass, user, target_ip, file_name);
	println!("{}", ssh_command);    
    //command running settings
    let mut output = Command::new("sh")
        .arg("-c")
        .arg(&ssh_command)
        .spawn()
        .expect("Failed to execute bash");
    //parse exit status
    let result = output.wait().expect("Failed to wait for command");
    let exit_status = result.code().unwrap_or(-1); 
    let result_str = exit_status.to_string(); 
    match result_str.as_str() {
        "1" => {
            println!("Something's wrong, the commands could not be executed. Check the SSH server integrity.");
        }
        "0" => {
            println!("Initial SSH worked! Verifying success...");
        }
        "-1" => {
            println!("Problem with unwrapping!");
        }
        &_ => {
            println!("Unexpected result!");
        }
    }

    //Verify
    let other_output = Command::new("sh")
        .arg("-c")
        .arg(&check_command)
        .output()
        .expect("Failed to execute bash");
    let verify_output = String::from_utf8_lossy(&other_output.stdout);
    if kill_status {
        if verify_output == "" {
            println!("Success verified! Process killed successfully.");
        } else {
            println!("Some error has occured; process is still running.");
        }
    } else {
        if verify_output != "" {
            println!("Success verified! Process is running.");
        } else {
            println!("Some error has occured; process is not running.");
    }
    }
}
