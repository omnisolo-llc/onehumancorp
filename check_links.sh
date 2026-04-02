#!/bin/bash
find docs/ -name "*.md" -print0 | while IFS= read -r -d '' file; do
    grep -oP '\[.*?\]\(\K[^)]+(?=\))' "$file" | grep -v '^http' | grep -v '^#' | while read -r link; do
        dir=$(dirname "$file")
        target="$dir/$link"
        if [ ! -e "$target" ]; then
            echo "Broken link in $file: $link"
        fi
    done
done
