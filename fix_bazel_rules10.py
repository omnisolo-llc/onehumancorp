import re
filepath = "/home/jules/.cache/bazel/_bazel_jules/8c069df52082beee3c95ca17836fb8e2/external/rules_flutter+/flutter/private/package_generation.bzl"

with open(filepath, "r") as f:
    content = f.read()

# Fix syntax error
content = content.replace("            return False\n        repository_ctx.file(pub_deps_file, '{\\\"packages\\\": []}').\\nstdout: {stdout}\\nstderr: {stderr}\".format(\n            tool = tool,\n            pkg = package_name,\n            dir = package_dir,\n            stdout = deps_result.stdout,\n            stderr = stderr,\n        ))\n", "            return False\n        repository_ctx.file(pub_deps_file, '{\\\"packages\\\": []}')\n        return True\n")

with open(filepath, "w") as f:
    f.write(content)
