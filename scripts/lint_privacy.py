#!/usr/bin/env python3
import sys
import os
import re

def check_file(filepath):
    try:
        with open(filepath, 'r') as f:
            content = f.read()
            # Strip comments for simple validation
            content_no_comments = re.sub(r'//.*', '', content)
            content_no_comments = re.sub(r'/\*.*?\*/', '', content_no_comments, flags=re.DOTALL)

            if 'PgPoolOptions' in content_no_comments and 'DISCARD ALL' not in content_no_comments:
                print(f"Error: Found PgPoolOptions without DISCARD ALL in {filepath}")
                return False
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
    return True

def main():
    success = True
    files_checked = 0
    workspace_dir = os.environ.get('BUILD_WORKSPACE_DIRECTORY', '.')
    src_dir = os.path.join(workspace_dir, 'src')

    if not os.path.exists(src_dir):
        src_dir = 'src'

    for root, _, files in os.walk(src_dir):
        for file in files:
            if file.endswith('.rs') or file.endswith('.go'):
                filepath = os.path.join(root, file)
                if not check_file(filepath):
                    success = False
                files_checked += 1

    print(f"Checked {files_checked} files for privacy compliance.")
    if not success:
        sys.exit(1)
    print("Linting passed.")
    sys.exit(0)

if __name__ == "__main__":
    main()
