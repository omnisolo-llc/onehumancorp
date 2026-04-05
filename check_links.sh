#!/bin/bash
# Simple bash script to check relative links in markdown files

broken=0
while IFS= read -r match; do
  file=$(echo "$match" | cut -d':' -f1)
  link=$(echo "$match" | grep -oP "\]\(\K[^)]+")

  # Ignore http/https
  if [[ "$link" == http* ]]; then continue; fi

  # Strip fragment #
  link_path=$(echo "$link" | cut -d'#' -f1)

  # If link is empty after stripping fragment, it's just a same-page anchor, skip
  if [[ -z "$link_path" ]]; then continue; fi

  dir=$(dirname "$file")
  target="$dir/$link_path"

  if [ ! -f "$target" ] && [ ! -d "$target" ]; then
    echo "Broken link in $file: $link (resolved to $target)"
    broken=$((broken+1))
  fi
done < <(grep -roE "\[[^]]+\]\([^)]+\)" docs/)

if [ $broken -gt 0 ]; then
  echo "Found $broken broken links!"
else
  echo "All relative links are valid."
fi
