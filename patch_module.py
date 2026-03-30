with open("MODULE.bazel", "r") as f:
    code = f.read()

import re
code = re.sub(r'single_version_override\(\n    module_name = "aspect_bazel_lib",\n    patch_cmds = \[\n.*?\n    \],\n\)\n+', '', code, flags=re.DOTALL)

with open("MODULE.bazel", "w") as f:
    f.write(code)
