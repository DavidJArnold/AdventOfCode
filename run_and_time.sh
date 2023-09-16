#!/bin/bash

# Function to check if two files are similar
function files_are_similar {
    diff -qwB "$1" "$2" &>/dev/null
    return $?  # Return the exit status of the diff command
}

python_runtime=0
rust_runtime=0

# Iterate through all subdirectories
for dir in */; do
    dir=${dir%/}  # Remove trailing slash

    # Check if a Python script with the same name exists
    if [ -f "$dir/$dir.py" ]; then
        # Check if a Rust program with the same name exists
        if [ -f "$dir/src/main.rs" ]; then

            cd $dir
            start_time=$(date +%s.%N)
            python_output=$(python3 "$dir.py")
            end_time=$(date +%s.%N)
            elapsed_time_python=$(echo "$end_time - $start_time" | bc)
            python_runtime=$(echo "$python_runtime + $elapsed_time_python" | bc)

            cargo build -r &>/dev/null
            start_time=$(date +%s.%N)
            rust_output=$("./target/release/day_$dir")
            end_time=$(date +%s.%N)
            elapsed_time_rust=$(echo "$end_time - $start_time" | bc)
            rust_runtime=$(echo "$rust_runtime + $elapsed_time_rust" | bc)
            cd ..

            echo "Day $dir"
            echo "Python: ${elapsed_time_python}s"
            echo "Rust:   ${elapsed_time_rust}s"

            if files_are_similar <(echo "$python_output") <(echo "$rust_output"); then
                :
            else
                echo "Python and Rust outputs are different in '$dir' directory."
                echo "$python_output"
                echo "$rust_output"
            fi
        fi
    fi
done

echo "-----------------------------"
echo "Python total runtime: ${python_runtime}s"
echo "Rust total runtime: ${rust_runtime}s"
saved=$(echo "100 * ($python_runtime - $rust_runtime) / $python_runtime" | bc -l)
rounded=$(printf "%.2f" "$saved")
speedup=$(echo "$python_runtime / $rust_runtime" | bc -l)
rounded_speedup=$(printf "%.2f" "$speedup")
echo "Time saving: $rounded% (${rounded_speedup}x speed increase)"
