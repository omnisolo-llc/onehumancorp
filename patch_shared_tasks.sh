#!/bin/bash
sed -i "s/shared_tasks_v4/shared_tasks_decomposition/g" srcs/server/orchestration/shared_tasks.go
sed -i "s/shared_tasks_v4/shared_tasks_decomposition/g" srcs/server/orchestration/shared_tasks_test.go
