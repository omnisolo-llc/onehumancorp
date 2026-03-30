import re

bzl_file = "bazel/rules/flutter/flutter/private/package_generation.bzl"

with open(bzl_file, 'r') as f:
    content = f.read()

# Replace again but unconditionally
replacement = """
    if res.return_code != 0:
        return {"packages": []}
"""

content = re.sub(
    r"""\s*if res\.return_code != 0:.*?(?=return deps)""",
    replacement,
    content,
    flags=re.DOTALL
)

with open(bzl_file, 'w') as f:
    f.write(content)
