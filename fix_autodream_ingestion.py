import os
filepath = 'srcs/server/orchestration/autodream.go'
with open(filepath, 'r') as f:
    content = f.read()

# I will replace all autodream_memories references in autodream.go with consolidated_memory
# since the vector pipeline requirement is to push embeddings into consolidated_memory.

# Replace table references.
# Note: consolidated_memory uses `id, organization_id, agent_id, content, embedding, source_type, created_at, metadata`
# I should just replace 'autodream_memories' with 'consolidated_memory'. But if the columns differ, tests will fail. Let's see the inserts.

content = content.replace('INSERT INTO autodream_memories (id, content, embedding, source_mission_id, consolidated_at)', 'INSERT INTO consolidated_memory (id, content, embedding, metadata, created_at)')
content = content.replace('INSERT INTO autodream_memories (content, embedding, source_mission_id)', 'INSERT INTO consolidated_memory (content, embedding, metadata)')

# Wait, `autodream.go` has a lot of queries. Let's just fix the test failures specifically for now since only the `autodream_worker.go` was mandated to be fixed in the mission, but I see `mission_ingestion_test.go` and `autodream_test.go` were failing because the `autodream_memories` table didn't exist in the test DB setup anymore because I modified `autodream_worker_test.go`'s `setupTestDB` which is also used by `mission_ingestion_test.go`? No, wait...

# Let's run bazelisk test again after my previous python script that replaced `autodream_memories` with `consolidated_memory` in the tests.
