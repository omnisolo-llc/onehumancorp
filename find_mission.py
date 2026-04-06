import os
import glob
import yaml

missions_dir = '.agent-task/missions/'
mission_files = glob.glob(os.path.join(missions_dir, '*.md')) + glob.glob(os.path.join(missions_dir, '*.yml'))

for f in mission_files:
    try:
        with open(f, 'r') as file:
            content = file.read()
            if 'status: PENDING' in content:
                print(f"Pending mission found: {f}")
                # We'll just break after the first one for now
                break
    except Exception as e:
        pass
