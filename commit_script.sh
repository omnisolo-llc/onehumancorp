#!/bin/bash
git fetch origin main && git checkout -b autodream_clean_fix origin/main
git config user.email "jules@onehumancorp.com"
git config user.name "Jules"
python3 patch_main_go.py
python3 patch_workers.py
git add srcs/server/main.go srcs/server/workers/autodream_worker.go
git commit -m "🧹 Maintainer: Initialize and start AutoDream worker from workers package"
git diff origin/main
