import re

with open('srcs/server/orchestration/autodream_pipeline_test.go', 'r') as f:
    content = f.read()

# Fix table creation logic
old_code = """	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)"""

new_code = """"""

if old_code in content:
    content = content.replace(old_code, new_code)
    with open('srcs/server/orchestration/autodream_pipeline_test.go', 'w') as f:
        f.write(content)
    print("Patched test successfully")
