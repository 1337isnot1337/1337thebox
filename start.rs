use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};





fn main() {
    let args: Vec<String> = env::args().collect();

    let user = &args[1];
    let pass = &args[2];
    let yourip = &args[3];
    let otherip = &args[4];

}


fn pythonpidfind() {
    // run the ps auxf command, listing all processes running
    let output = Command::new("ps")
        .args(&["auxf"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute ps auxf, idk how to fix");

    let reader = BufReader::new(output.stdout.unwrap());
    let mut python_pid = String::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.contains(&format!("[p]ython3 -m http.server 55203")) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            python_pid = parts[1].to_string();
            break;
        }
    }
}


use std::process::Command;

fn pythonstart() {
    let port = "55203"; // specify the port

    // Spawn the Python server process in the background
    Command::new("python3")
        .args(&["-m", "http.server", port])
        .spawn()
        .expect("Failed to start Python server");
    println!("Python server started on port {}", port);
}

