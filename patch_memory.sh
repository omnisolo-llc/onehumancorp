#!/bin/bash
git log -n 1 --pretty=format:%B > commit_msg.txt
if grep -q "AutoDream Vector Consolidation" commit_msg.txt; then
  echo "Found correct context."
fi
