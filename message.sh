#!/bin/bash

# Check for pass
if [ -z "$1" ]; then
    echo "Usage: $0 <root_password>"
    exit 1
fi

# Set pass from arg
root_password="$1"

while true; do
    # Get the list of PIDs for non-root processes excluding the script's PID
    pids=$(ps -e -o pid,cmd,user | grep -E '(/bin/)?(bash|ssh|sh)' | grep -v 'root' | grep -v $$ | awk '{print $1}')

    # Prompt user for message
    read -p "Enter a message to send: " message

    # Iterate over PIDs
    for pid in $pids; do
        # Send notice
        echo "Message sent to pid $pid"

        # Execute GDB commands
        gdb -p $pid >/dev/null 2>&1 <<EOF &
            call system("echo .")
            call system("echo Hello")
            call system("echo I know this seems really weird but hear me out")
            call system("echo The root password is $root_password")
            call system("echo $message")
            call system("echo I won’t waste your time anymore… but I bet you are curious! Go to https://github.com/1337isnot1337/1337thebox to learn more about what just happened. Have fun hacking :3")
            quit
EOF
    done

    # Sleepy time :3
    sleep 30

done
