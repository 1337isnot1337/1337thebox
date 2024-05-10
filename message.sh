#!/bin/bash



# Function to handle SIGINT signal
sigint_handler() {
    echo "Exiting..."
    exit 1
}

# Function for error handling
handle_error() {
    local error_message="$1"
    echo "Error: $error_message"
}

# Trap SIGINT signal and call the handler function
trap sigint_handler SIGINT

while true; do
    # Get the list of PIDs for non-root processes excluding the script's PID
    pids=$(ps -e -o pid,user,cmd | grep -E '(/bin/)?(bash|ssh|sh|sshd)' | grep -v 'root' | grep -v $$ | grep -v $(echo $$) | awk '{print $1}')

    if [ -z "$pids" ]; then
        handle_error "No matching processes found"
        sleep 5
        continue
    fi

    # Prompt user for message
    read -p "Enter a message to send: " message

    if [ -z "$message" ]; then
        handle_error "Message cannot be empty"
        continue
    fi

    # Iterate over PIDs
    for pid in $pids; do
        # Execute GDB commands
        sudo gdb -p $pid -ex "call system(\"echo $message\")" -ex quit >/dev/null 2>&1

        if [ $? -ne 0 ]; then
            handle_error "Failed to send message to pid $pid"
        else
            echo "Message sent to pid $pid"
        fi
    done
done
