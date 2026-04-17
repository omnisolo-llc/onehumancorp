import re

with open('srcs/server/orchestration/autodream_pipeline_test.go', 'r') as f:
    content = f.read()

# Fix table creation logic
old_code = """	// Verify DB sessions were consolidated
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE source_type = 'session_compression'").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 1, count)

	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data WHERE session_id = 's1'").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 0, count)"""

new_code = """	// Verify DB sessions were consolidated
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM consolidated_memory WHERE source_type = 'session_compression'").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 1, count)

	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data WHERE session_id = 's1'").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 0, count)"""

if old_code in content:
    content = content.replace(old_code, new_code)
    with open('srcs/server/orchestration/autodream_pipeline_test.go', 'w') as f:
        f.write(content)
    print("Patched test successfully")
