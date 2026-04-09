#!/bin/bash
sed -i 's/TaskOrchestrator/SharedTaskDBManager/g' srcs/server/orchestration/tasks_db_test.go
sed -i 's/TaskOrchestrator/SharedTaskDBManager/g' srcs/server/orchestration/tasks_db_ext.go
