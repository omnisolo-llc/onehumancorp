import re

def main():
    filename = ".agent-task/missions/2026-04-05T14-03-15Z_standalone_metric_buffer.md"
    with open(filename, 'r') as f:
        content = f.read()

    content = re.sub(r'status: PENDING', r'status: DONE\nagent: Jules', content)

    with open(filename, 'w') as f:
        f.write(content)

if __name__ == "__main__":
    main()
