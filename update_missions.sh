#!/bin/bash
for file in .agent-task/missions/*.md .agent-task/missions/*.yml; do
  if [ -f "$file" ]; then
    if grep -q 'status: "PENDING"' "$file"; then
      sed -i 's/status: "PENDING"/status: PENDING/' "$file"
    fi
  fi
done
