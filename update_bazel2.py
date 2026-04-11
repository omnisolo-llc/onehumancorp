with open("srcs/server/orchestration/BUILD.bazel", "r") as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "autodream_worker.go" in line or "autodream_worker_test.go" in line:
        continue
    new_lines.append(line)

with open("srcs/server/orchestration/BUILD.bazel", "w") as f:
    f.writelines(new_lines)
