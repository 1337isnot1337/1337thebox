use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
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
    let pythonpid = pythonpidfind();
    thread::sleep(Duration::from_secs(10));
    pythonpidkill(pythonpid);

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

fn pythonpidfind() -> i32 {

    let output = Command::new("ps")
        .args(&["auxf"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute ps auxf, idk how to fix");

    let reader = BufReader::new(output.stdout.unwrap());
    let mut python_pid = String::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.contains("[p]ython3 -m http.server 55203") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            python_pid = parts[1].to_string();
            println!("The PID of the python process is {}", python_pid);
            return python_pid.parse().expect("Failed to parse PID");
        }
    }
    panic!("Python process not found");
}

fn pythonstart() {
    let port = "55203"; 

 
    Command::new("python3")
        .args(&["-m", "http.server", port])
        .spawn()
        .expect("Failed to start Python server");
    println!("Python server started on port {}", port);
}
