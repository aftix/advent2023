#!/bin/sh

perl -n -e'/name = "(day([1-9][0-9]|[1-9])(p2)?)"/ && print $1,"\n"' < Cargo.toml |\
  while IFS= read -r day; do 
    FILE="src/bin/$day.rs"
    mkdir -p "src/bin"
    echo "use std::io;" > "$FILE"
    echo >> "$FILE"
    echo "fn main() {" >> "$FILE"
    echo "    let lines: Vec<String> = io::stdin().lines().map_while(Result::ok).collect();" >> "$FILE"
    echo "    let input: Vec<&str> = lines.iter().map(String::as_str).collect();" >> "$FILE"
    echo "    println!(\"Result is {}\", advent2023::$day(&input));" >> "$FILE"
    echo "}" >> "$FILE"
  done
