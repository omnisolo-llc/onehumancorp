import glob

files = glob.glob(".agent-task/missions/*.md") + glob.glob(".agent-task/missions/*.yml")

for f in files:
    with open(f, 'r') as file:
        content = file.read()
    if 'status: PENDING' in content or 'status: OPEN' in content:
        print(f"Pending/Open file found: {f}")
