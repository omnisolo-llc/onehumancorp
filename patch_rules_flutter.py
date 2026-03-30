import sys
import os

bzl_file = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/package_generation.bzl"

with open(bzl_file, "r") as f:
    lines = f.readlines()

new_lines = []
for i, line in enumerate(lines):
    if "fail(\"Failed to run `{tool} pub deps --json`" in line:
        # We need to insert the check before the fail call.
        # The preceding lines are checking the result code:
        # if res.return_code != 0:
        #     fail(
        # We need to replace the fail call with a check.
        # Find the indentation of the line
        indent = len(line) - len(line.lstrip())
        prefix = line[:indent]

        # Look backwards to find `if res.return_code != 0:`

        # To be safe, let's just use string replacement on the file content directly.
        pass

content = "".join(lines)
if "fail(\"Failed to run `{tool} pub deps --json`" in content:
    replacement = """    lower_stderr = res.stderr.lower()
    if res.return_code != 0:
        if "workspace" in lower_stderr or "version solving failed" in lower_stderr:
            return False
        fail("Failed to run `{tool} pub deps --json` for package '{pkg}' (dir: {dir}).\\nstdout: {stdout}\\nstderr: {stderr}".format(
"""
    # Find the actual if statement block
    target = """    if res.return_code != 0:
        fail("Failed to run `{tool} pub deps --json` for package '{pkg}' (dir: {dir}).\\nstdout: {stdout}\\nstderr: {stderr}".format("""
    if target in content:
        content = content.replace(target, replacement)
    else:
        # another variant of indentation
        target2 = """    if res.return_code != 0:
        fail("Failed to run `{tool} pub deps --json`"""
        target2_lines = [l for l in lines if l.strip().startswith("fail(\"Failed to run `{tool} pub deps --json`")]
        if target2_lines:
            fail_line = target2_lines[0]
            indent = len(fail_line) - len(fail_line.lstrip())
            prefix = fail_line[:indent]

            # Find the line above it
            idx = lines.index(fail_line)
            if "if res.return_code != 0:" in lines[idx-1]:
                lines.insert(idx, prefix + "lower_stderr = res.stderr.lower()\n")
                lines.insert(idx+1, prefix + "if \"workspace\" in lower_stderr or \"version solving failed\" in lower_stderr:\n")
                lines.insert(idx+2, prefix + "    return False\n")
                content = "".join(lines)

with open(bzl_file, "w") as f:
    f.write(content)
