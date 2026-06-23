import os
with open("src/server/api/BUILD.bazel", "r") as f:
    lines = f.readlines()
new_lines = []
for line in lines:
    new_lines.append(line)
    if '"unified_inbox_webhook.rs",' in line and 'work_triage' not in "".join(lines):
        new_lines.append('        "work_triage.rs",\n')
with open("src/server/api/BUILD.bazel", "w") as f:
    f.writelines(new_lines)
