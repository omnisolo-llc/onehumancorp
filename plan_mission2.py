import os

def get_missions():
    directory = ".agent-task/missions/"
    for filename in os.listdir(directory):
        if not filename.endswith(".md"):
            continue
        filepath = os.path.join(directory, filename)
        with open(filepath, 'r') as f:
            content = f.read()
            if "status: PENDING" in content or "status: \"PENDING\"" in content:
                print(f"Mission: {filename}")
                # Print first 20 lines
                print("\n".join(content.split("\n")[:20]))
                print("-" * 40)
