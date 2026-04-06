import sys, re

def update_status(filename, agent_name):
    with open(filename, 'r') as f:
        content = f.read()

    # Just prepend status and agent if not present or replace
    if "status: DONE" in content:
        print(f"File {filename} is already marked DONE.")
        return

    # Replace IN_PROGRESS with DONE
    if "status: IN_PROGRESS" in content:
        content = content.replace("status: IN_PROGRESS", "status: DONE")
        with open(filename, 'w') as f:
            f.write(content)
        print(f"Updated {filename} to DONE")
        return

    # Or insert at top
    new_content = f"---\nstatus: DONE\nagent: {agent_name}\n---\n\n" + content

    with open(filename, 'w') as f:
        f.write(new_content)
    print(f"Updated {filename} to DONE for {agent_name}")

if __name__ == "__main__":
    update_status(sys.argv[1], sys.argv[2])
