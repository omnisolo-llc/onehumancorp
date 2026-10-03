#!/bin/bash
skill_name=$1
git clone https://github.com/obra/superpowers.git .agent-scratch/superpowers 2>/dev/null || true
cd .agent-scratch/superpowers
git rev-parse HEAD
cat skills/$skill_name/SKILL.md
