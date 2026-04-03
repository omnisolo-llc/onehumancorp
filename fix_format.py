with open('srcs/server/orchestration/tasks.go', 'r') as f:
    lines = f.readlines()

new_lines = []
skip_next_empty = False
for line in lines:
    if line.strip() == '':
        if skip_next_empty:
            continue
        skip_next_empty = True
    else:
        skip_next_empty = False
    new_lines.append(line)

with open('srcs/server/orchestration/tasks.go', 'w') as f:
    f.writelines(new_lines)
