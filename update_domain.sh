#!/bin/bash

# Find files inside the allowed domain that contain the string "NewSQLiteProvider"
files=$(find tests/ monitoring/ lib/resilience/ -type f -name "*.go" 2>/dev/null)

if [ -n "$files" ]; then
  for file in $files; do
    sed -i 's/NewSQLiteProvider/NewSqliteProvider/g' "$file"
  done
fi
