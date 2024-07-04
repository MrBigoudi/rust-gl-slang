#!/bin/bash

# Set the directory where your Slang shaders are located
SHADER_DIR="src/shaders"

# Set the path to the slangc executable
SLANGC="slangc"

# Compile all .slang shaders in the SHADER_DIR
for file in "$SHADER_DIR"/*.slang; do
    if [ -f "$file" ]; then
        output_file="${file%.slang}.glsl"
        "$SLANGC" "$file" -profile glsl_460 -target glsl -o "$output_file" -entry main
        echo "Compiled $file to $output_file"
    fi
done
