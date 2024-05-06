use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, self, Read};
use std::{thread, env};
use std::time::Duration;


fn main() {
    let args: Vec<String> = env::args().collect();

    let user = &args[1];
    let pass = &args[2];
    let yourip = &args[3];
    let otherip = &args[4];
    println!("{},{},{},{}", user, pass, yourip, otherip);
    
    pythonstart();
    let pythonpid: i32 = pythonpidfind() as i32;
    thread::sleep(Duration::from_secs(10));
    pythonpidkill(pythonpid);

}

fn sshupload() {
    
}

fn pythonpidkill(pid: i32) {
    let output = Command::new("kill")
        .args(&[&format!("{}", pid)])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error in killing process")
        .wait()
        .expect("Failed to wait for kill command");
}

fn pythonpidfind() -> u32 {
    // Run the ps auxf command, listing all processes running
    let output = Command::new("ps")
        .args(&["auxf"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute ps auxf");

    let reader = BufReader::new(output.stdout.unwrap());
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.contains("[p]ython3 -m http.server 55203") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 1 {
                let python_pid = parts[1].parse().expect("Failed to parse PID");
                println!("The PID of the python process is {}", python_pid);
                return python_pid;
            } else {
                panic!("PID not found");
            }
        }
    }
    panic!("Python process not found");
}


fn pythonstart() {
    let port = "55203"; // specify the port

    // Spawn the Python server process in the background
    Command::new("python3")
        .args(&["-m", "http.server", port])
        .spawn()
        .expect("Failed to start Python server");
    println!("Python server started on port {}", port);
}
