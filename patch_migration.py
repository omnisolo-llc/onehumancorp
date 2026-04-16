with open("srcs/server/db/migrations/033_swarm_tasks_kairos.sql", "r") as f:
    content = f.read()

content = content.replace("ALTER TABLE swarm_tasks ADD COLUMN IF NOT EXISTS organization_id TEXT;", "ALTER TABLE swarm_tasks ADD COLUMN organization_id TEXT;")
content = content.replace("ALTER TABLE swarm_tasks DROP COLUMN IF EXISTS organization_id;", "ALTER TABLE swarm_tasks DROP COLUMN organization_id;")

with open("srcs/server/db/migrations/033_swarm_tasks_kairos.sql", "w") as f:
    f.write(content)
