WIP for now

Overview:

This simple lil script will send an alert to all bash shells it finds, telling them the root password. 


Purpose:

To be really annoying :3


Installation

1. `git clone https://github.com/1337isnot1337/1337thebox`
2. cd 1337thebox/
3. If you don't have rust, `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` to install
4. `rustc 1337thebox.rs`
5. `chmod +x 1337thebox`


Usage:

`./1337thebox {user} {user's password} {target IP} {path to the escalation file} {OPTIONAL: kill}`


examples:

`./1337thebox root qwerty123 10.10.132.177 escalate.sh`
^ starts the script 'escalate.sh' as user 'root' at '10.10.132.177' with password 'qwerty123' ^

`./1337thebox root hamborgzer 10.10.111.78 supacool.sh kill`
^ kills the script 'supacool.sh' as user 'root' at '10.10.111.78' with password 'hamborgzer' ^



