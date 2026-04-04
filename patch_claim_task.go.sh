#!/bin/bash
cat srcs/server/orchestration/tasks.go | grep -A 40 "func (tm \*TaskManager) ClaimTask"
