#!/bin/bash
for file in .agent-task/missions/*.md; do
    if ! grep -q "status: DONE\|status: IN_PROGRESS\|status: BLOCKED" "$file"; then
        echo "$file"
    fi
done
