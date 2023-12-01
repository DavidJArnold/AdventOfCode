#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi
base_directory="$1"

if [ ! -d "$base_directory" ]; then
    echo "Error: '$base_directory' is not a directory"
    exit 1
fi

# Function to check if two files are similar
function files_are_similar {
    diff -qwB "$1" "$2" &>/dev/null
    return $?  # Return the exit status of the diff command
}

python_runtime=0
rust_release_runtime=0
rust_debug_runtime=0

# Iterate through all subdirectories
for dir in "$base_directory"/*/; do
    dir=${dir%/}  # Remove trailing slash
    subdir=$(basename $dir)

    # Check if a Python script with the same name exists
    if [ -f "$dir/$subdir.py" ]; then
        # Check if a Rust program with the same name exists
        if [ -f "$dir/src/main.rs" ]; then

            cd $dir
            start_time=$(date +%s.%N)
            python_output=$(python3 "$subdir.py")
            end_time=$(date +%s.%N)
            elapsed_time_python=$(echo "$end_time - $start_time" | bc)

            cargo build -r &>/dev/null
            start_time=$(date +%s.%N)
            rust_output=$("./target/release/day_$subdir")
            end_time=$(date +%s.%N)
            elapsed_time_rust_release=$(echo "$end_time - $start_time" | bc)

            cargo build &>/dev/null
            start_time=$(date +%s.%N)
            rust_output=$("./target/debug/day_$subdir")
            end_time=$(date +%s.%N)
            elapsed_time_rust_debug=$(echo "$end_time - $start_time" | bc)

            cd ../..

            echo "Day $subdir"
            if files_are_similar <(echo "$python_output") <(echo "$rust_output"); then
                python_runtime=$(echo "$python_runtime + $elapsed_time_python" | bc)
                rust_release_runtime=$(echo "$rust_release_runtime + $elapsed_time_rust_release" | bc)
                rust_debug_runtime=$(echo "$rust_debug_runtime + $elapsed_time_rust_debug" | bc)
                echo "Python: ${elapsed_time_python}s"
                echo "Rust R:   ${elapsed_time_rust_release}s"
                echo "Rust D:   ${elapsed_time_rust_debug}s"
            else
                echo "Python and Rust outputs are different in '$dir' directory."
                echo "Python:"
                echo "$python_output"
                echo "Rust:"
                echo "$rust_output"
            fi
        fi
    fi
done

echo "-----------------------------"
echo "Python total runtime:       ${python_runtime}s"
echo "Rust debug total runtime:   ${rust_debug_runtime}s"
echo "Rust release total runtime: ${rust_release_runtime}s"
saved=$(echo "100 * ($python_runtime - $rust_release_runtime) / $python_runtime" | bc -l)
rounded=$(printf "%.2f" "$saved")
speedup=$(echo "$python_runtime / $rust_release_runtime" | bc -l)
rounded_speedup=$(printf "%.2f" "$speedup")
echo "Time saving: $rounded% (${rounded_speedup}x speed increase)"
