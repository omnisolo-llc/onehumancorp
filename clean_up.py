import os
for path in [
    'srcs/server/db/BUILD.bazel',
    'srcs/server/orchestration/BUILD.bazel',
    'srcs/server/orchestration/tasks_store.go',
    'srcs/server/orchestration/tasks_store_test.go',
    '.agent-task/missions/2026-04-14T14-49-50Z.md'
]:
    with open(path, 'r') as f:
        lines = f.readlines()

    clean_lines = []
    skip = False
    for line in lines:
        if line.startswith('<<<<<<<'):
            skip = True
        elif line.startswith('======='):
            skip = False
        elif line.startswith('>>>>>>>'):
            skip = False
        elif not skip:
            clean_lines.append(line)

    with open(path, 'w') as f:
        f.writelines(clean_lines)
