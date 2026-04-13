import re

with open("srcs/server/orchestration/sip.go", "r") as f:
    content = f.read()

# Add synced_to_cloud to agent_missions CREATE TABLE
new_schema = """`CREATE TABLE IF NOT EXISTS agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT DEFAULT 'system',
			synced_to_cloud BOOLEAN DEFAULT FALSE
		);`"""

content = re.sub(r'`CREATE TABLE IF NOT EXISTS agent_missions \(\n\t\t\tid TEXT PRIMARY KEY,\n\t\t\tstatus TEXT NOT NULL,\n\t\t\tpayload TEXT NOT NULL,\n\t\t\tcreated_at DATETIME DEFAULT CURRENT_TIMESTAMP,\n\t\t\torganization_id TEXT DEFAULT \'system\'\n\t\t\);`', new_schema, content)

with open("srcs/server/orchestration/sip.go", "w") as f:
    f.write(content)
