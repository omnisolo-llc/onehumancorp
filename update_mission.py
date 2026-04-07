import sys
import re

def main():
    filename = sys.argv[1]
    with open(filename, 'r') as f:
        content = f.read()

    # Simple replace
    new_content = re.sub(r'status:\s*"?PENDING"?', 'status: "IN_PROGRESS"\nagent: Jules', content)

    with open(filename, 'w') as f:
        f.write(new_content)
    print("Updated", filename)

if __name__ == "__main__":
    main()
