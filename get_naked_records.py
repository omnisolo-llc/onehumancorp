import re

with open("srcs/server/telemetry/telemetry.go", "r") as f:
    content = f.read()

# Find occurrences of .Add(ctx, X) or .Record(ctx, X) WITHOUT WithAttributes
add_naked = re.findall(r'\.Add\(ctx, [^\)]+\)(?!\s*,)', content)
record_naked = re.findall(r'\.Record\(ctx, [^\)]+\)(?!\s*,)', content)

print("Naked Add:", add_naked)
print("Naked Record:", record_naked)
