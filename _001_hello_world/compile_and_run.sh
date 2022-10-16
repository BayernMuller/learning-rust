#!/bin/bash

# I couldn't use "date '+%s.%N'" because it's not supported on MacOS
function get_current_time()
{
    python3 -c "import time; print(time.time())"
}

output_file="hello_world"

compile_command="rustc main.rs -o $output_file"

echo "[$(date)] Compiling..."
compile_beginning_time=$(get_current_time)

$compile_command
compile_result=$?

compile_ending_time=$(get_current_time)

if [ "$compile_result" -eq 0 ]; then
    echo "[$(date)] Compilation successful!"
else
    echo "[$(date)] Compilation failed!"
    exit 1
fi

compile_duration=$(echo "$compile_ending_time - $compile_beginning_time" | bc | sed 's/^\./0./')
echo "[$(date)] Compilation done in $compile_duration seconds."

echo "[$(date)] Running..."

./$output_file