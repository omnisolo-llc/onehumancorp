import sys
import re

with open("srcs/server/db/BUILD.bazel", "r") as f:
    data = f.read()

# Make sure "migrations/052_kairos_foundation.sql" is inside embedsrcs
if "052_kairos_foundation.sql" not in data:
    pattern = r'(embedsrcs\s*=\s*\[.*?)(\n\s*\])'
    replacement = r'\1\n        "migrations/052_kairos_foundation.sql",\2'
    data = re.sub(pattern, replacement, data, flags=re.DOTALL)
    with open("srcs/server/db/BUILD.bazel", "w") as f:
        f.write(data)

print("success")
