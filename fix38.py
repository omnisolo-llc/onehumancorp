# Check what JSON fields `handoff.Agent` has!
with open('srcs/handoff/types.go', 'r') as f:
    content = f.read()

print("Handoff types:")
print(content[:500])
