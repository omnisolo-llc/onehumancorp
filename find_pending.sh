for file in .agent-task/missions/*; do
  if grep -q -i -E "^status:\s*\"*PENDING" "$file"; then
    echo "$file"
  fi
done
