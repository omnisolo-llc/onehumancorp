import re

with open('srcs/server/standalone_ohc.sh', 'r') as f:
    content = f.read()

search = """    GOGC="${GOGC:-30}" \\
    OHC_STANDALONE="true" \\"""
replace = """    GOGC="${GOGC:-30}" \\
    OHC_STANDALONE="true" \\
    OHC_SQLITE_KEY="${OHC_SQLITE_KEY:-standalone_ephemeral_key}" \\"""

if search in content:
    with open('srcs/server/standalone_ohc.sh', 'w') as f:
        f.write(content.replace(search, replace))
    print("Fixed standalone_ohc.sh")
else:
    print("Search string not found in standalone_ohc.sh")
