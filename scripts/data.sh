#!/bin/bash

# Function to create Python solution files
create_python_solution() {
    local year=$1
    local day=$2
    local day_dir=$3
    
    if [ ! -s "$day_dir/part1.py" ] && [ ! -s "$day_dir/part2.py" ]; then
        rd="./$year/day$day"
        printf "$(cat "$wd/templates/template.py.txt")" $year $day 1 "$rd/input.txt" "$rd/result1.txt" > "$day_dir/part1.py"
        printf "$(cat "$wd/templates/template.py.txt")" $year $day 2 "$rd/input.txt" "$rd/result2.txt" > "$day_dir/part2.py"
    fi
}

# Function to create Rust solution files
create_rust_solution() {
    local year=$1
    local day=$2
    local day_dir=$3
    local project_name="aoc_${year}_${day}"

    # Initialize Cargo project if it doesn't exist
    if [ ! -f "$day_dir/Cargo.toml" ]; then
        cd "$day_dir"
        cargo init --name "$project_name"
        rm -f src/main.rs
        mkdir -p src/bin

        # create src/lib.rs as an (empty) common library file
        if [ ! -f src/lib.rs ]; then
            printf "// AoC %s day %s library\n\n" "$year" "$day" > src/lib.rs
        fi

        cd - > /dev/null
    fi

    # Create part1.rs binary
    if [ ! -s "$day_dir/src/bin/part1.rs" ]; then
        rd="../"
        printf "$(cat "$wd/templates/template.rs.txt")" $year $day 1 "$rd/input.txt" "$rd/result1.txt" > "$day_dir/src/bin/part1.rs"
    fi

    # Create part2.rs binary
    if [ ! -s "$day_dir/src/bin/part2.rs" ]; then
        rd="../"
        printf "$(cat "$wd/templates/template.rs.txt")" $year $day 2 "$rd/input.txt" "$rd/result2.txt" > "$day_dir/src/bin/part2.rs"
    fi
}

month=$(date +"%m")

if [ $month -ne 12 ]; then
    echo "It's not December, AoC hasn't started yet."
    exit
fi

hour=$(date +"%H")

if [ $hour -lt 6 ]; then
    echo "It's too early, today's challenge hasn't been posted yet."
    exit
fi

wd=$(dirname $0)/..

year=$(date +"%Y")
year_dir="$wd/$year"

if [ ! -d $year_dir ]; then
    mkdir "$year_dir"
fi 

if [ $# -eq 0 ]; then
    day=$(date +"%d")
    language="python"
elif [ $# -eq 1 ]; then
    day=$1
    language="python"
else
    day=$1
    language=$2
fi

day_dir="$year_dir/day$day"

if [ ! -d $day_dir ]; then
    mkdir "$day_dir"
    touch "$day_dir/input.txt"
    touch "$day_dir/example.txt"
    touch "$day_dir/result1.txt"
    touch "$day_dir/result2.txt"
fi

if [ ! -s "$day_dir/input.txt" ] && [ ! -s "$day_dir/example.txt" ]; then
    pipenv run aocd $year $day > "$day_dir/input.txt"
    pipenv run aocd $year $day --example > "$day_dir/example.txt"
fi

case $language in
    python|py)
        create_python_solution "$year" "$day" "$day_dir"
        ;;
    rust|rs)
        create_rust_solution "$year" "$day" "$day_dir"
        ;;
    *)
        echo "Unknown language: $language. Defaulting to Python."
        create_python_solution "$year" "$day" "$day_dir"
        ;;
esac

pipenv shell

