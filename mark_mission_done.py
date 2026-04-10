import re

with open('.agent-task/missions/1776500001_maintainer_expose_health_probes.yml', 'r') as f:
    content = f.read()

new_content = re.sub(r'status: .*', 'status: DONE', content)

with open('.agent-task/missions/1776500001_maintainer_expose_health_probes.yml', 'w') as f:
    f.write(new_content)
