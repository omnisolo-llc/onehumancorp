import re

with open("srcs/server/telemetry/telemetry.go", "r") as f:
    lines = f.readlines()

new_lines = []
skip = 0
for idx, line in enumerate(lines):
    if skip > 0:
        skip -= 1
        continue
    if "SyncLatency, err = m.Float64Histogram(" in line and idx == 217:
        skip = 6
        continue
    new_lines.append(line)

with open("srcs/server/telemetry/telemetry.go", "w") as f:
    f.writelines(new_lines)
