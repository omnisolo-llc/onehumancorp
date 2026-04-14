import re

with open("srcs/server/orchestration/sync_daemon.go", "r") as f:
    content = f.read()

# Replace:
# query := "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND status = 'CLOUD_ESCALATION' LIMIT 100"
# if d.dbWrapper.IsSQLite() {
# 	query = "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = 0 AND status = 'CLOUD_ESCALATION' LIMIT 100"
# }

new_query_code = """	query := "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND status = 'PENDING' LIMIT 500"
	if d.dbWrapper.IsSQLite() {
		query = "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = 0 AND status = 'PENDING' LIMIT 500"
	}"""

content = re.sub(r'query := "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false AND status = \'CLOUD_ESCALATION\' LIMIT 100"\n\tif d\.dbWrapper\.IsSQLite\(\) \{\n\t\tquery = "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = 0 AND status = \'CLOUD_ESCALATION\' LIMIT 100"\n\t\}', new_query_code, content)

with open("srcs/server/orchestration/sync_daemon.go", "w") as f:
    f.write(content)
