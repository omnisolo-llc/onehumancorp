# 1. We will remove the redundant migrations and keep only ONE migration with the correct schema
# We see 054_shared_tasks_decomposition.sql and 20260412_shared_tasks_decomposition.sql already exist.
# The prompt says to "Add a new migration".
# However, 20260412_shared_tasks_decomposition.sql is actually already in `srcs/server/db/migrations/` but NOT in BUILD.bazel.
# 054_shared_tasks_decomposition.sql is in BUILD.bazel but its schema is WRONG (uses VARCHAR instead of UUID, TEXT instead of JSONB).
# We also added 20260415_shared_tasks_decomposition_schema.sql.
# Let's drop our 20260415 migration and update 054_shared_tasks_decomposition.sql to match the requested schema exactly.
