#!/bin/bash
# Remove emojis from Rust source code files
# Emojis are prohibited in code for professional standards

set -e

echo "Removing emojis from Rust source files..."

# Find all .rs files and remove common emojis
find crates/ -name "*.rs" -type f -exec sed -i \
    -e 's/🚀//g' \
    -e 's/✅//g' \
    -e 's/❌//g' \
    -e 's/⚠️ //g' \
    -e 's/⚠️//g' \
    -e 's/🔒//g' \
    -e 's/💡//g' \
    -e 's/📝//g' \
    -e 's/📄//g' \
    -e 's/💾//g' \
    -e 's/🔍//g' \
    -e 's/🔓//g' \
    -e 's/⏳//g' \
    -e 's/🤖//g' \
    -e 's/📊//g' \
    -e 's/🛡️//g' \
    -e 's/💼//g' \
    -e 's/🐳//g' \
    {} \;

echo "Done! Emojis removed from source code."
echo "Total files processed:"
find crates/ -name "*.rs" -type f | wc -l
