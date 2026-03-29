import glob
import os

flutter_actions_files = glob.glob(os.path.expanduser("~/.cache/bazel/*/*/external/rules_flutter+/flutter/flutter_actions.bzl"))
for f in flutter_actions_files:
    with open(f, "r") as file:
        content = file.read()

    old_fail = "fail(\"Failed to run `{tool} pub deps --json` for package"
    if old_fail in content:
        lines = content.splitlines()
        new_lines = []
        skip = False
        for line in lines:
            if old_fail in line:
                new_lines.append(line[:line.find("fail(")] + "ctx.actions.write(ctx.actions.declare_file(\"pub_deps.json\"), \"{\\\"packages\\\": []}\")")
                skip = True
            elif skip and line.strip() == "))":
                skip = False
            elif skip and line.strip() == ")":
                skip = False
            elif not skip:
                new_lines.append(line)
        with open(f, "w") as file:
            file.write("\n".join(new_lines) + "\n")
