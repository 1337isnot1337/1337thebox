WIP for now

Overview
This simple lil script will send an alert to all bash shells it finds, telling them the root password. 

Purpose
To be really annoying :3


Installation
1. `git clone https://github.com/1337isnot1337/1337thebox`
2. cd 1337thebox/
3. If you don't have rust, `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` to install
4. `rustc start.rs`
5. `chmod +x start`

Usage:
`./start {user} {user's password} {target IP} {path to the escalation file}`
example:
`./start root qwerty123 10.10.132.177 /home/thebestuser/1337thebox/escalate.sh`
