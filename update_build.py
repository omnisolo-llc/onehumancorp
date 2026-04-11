import re
with open("srcs/server/db/BUILD.bazel", "r") as f:
    content = f.read()
pattern = r'"migrations/031_agent_missions_updated_at\.sql",'
replacement = '"migrations/031_agent_missions_updated_at.sql",\n        "migrations/032_autodream_memories_final.sql",'
content = re.sub(pattern, replacement, content)
with open("srcs/server/db/BUILD.bazel", "w") as f:
    f.write(content)
