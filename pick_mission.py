import os
import random

def get_missions():
    missions = []
    directory = ".agent-task/missions/"
    for filename in os.listdir(directory):
        if not filename.endswith(".md"):
            continue
        filepath = os.path.join(directory, filename)
        with open(filepath, 'r') as f:
            content = f.read()
            # Find onboarding pending tasks
            if "status: PENDING" in content or "status: \"PENDING\"" in content:
                missions.append(filename)
    return missions

if __name__ == "__main__":
    missions = get_missions()
    chosen = random.choice(missions)
    print("I am picking up the following mission:", chosen)
    with open(f".agent-task/missions/{chosen}", 'r') as f:
        print(f.read())
