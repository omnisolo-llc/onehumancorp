import re

with open("srcs/server/telemetry/telemetry.go", "r") as f:
    content = f.read()

# Find all metric recording methods
methods = re.findall(r'func Record\w+\(', content)
print(f"Total Record methods: {len(methods)}")
for method in methods:
    print(method)
