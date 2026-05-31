#!/bin/bash
kill $(lsof -t -i :3000) 2>/dev/null || true
export PORT=3000
export HOST=0.0.0.0
npm run build > /tmp/next_build.log 2>&1
npm run start > /tmp/next_preview.log 2>&1 &
