import os
import re

missions_dir = '.agent-task/missions/'
target_files = ['1775161662_scribe_mission_impl.yml', '1775218700_scribe_hybrid_arch.md', 'scribe_mission.yml', '1775216558_scribe_api_playbook.yml']

for filename in os.listdir(missions_dir):
    if filename in target_files:
        filepath = os.path.join(missions_dir, filename)
        with open(filepath, 'r') as f:
            content = f.read()

        # Ensure status is DONE
        content = re.sub(r'status: .*', 'status: DONE', content)

        with open(filepath, 'w') as f:
            f.write(content)

