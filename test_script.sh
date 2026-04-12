#!/bin/bash
echo "Plan steps:"
echo "1. Create migration file srcs/server/db/migrations/032_kairos_shared_tasks_v2.sql"
echo "2. Add to srcs/server/db/BUILD.bazel"
echo "3. Update ClaimTask in tasks_db.go to use shared_tasks_v2"
echo "4. Add ResolveTaskDependencies in tasks_db.go"
echo "5. Add logic to ultraplan.go"
echo "6. Update tests in tasks_db_test.go"
