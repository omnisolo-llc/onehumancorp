import sys

with open('lib/resilience/chaos/chaos_test.go', 'r') as f:
    content = f.read()

count = content.count("func TestCorruptAgentLock(")
if count > 1:
    idx1 = content.find("func TestCorruptAgentLock(")
    idx2 = content.find("func TestCorruptAgentLock(", idx1 + 1)

    end_idx = content.find("}", idx2) + 1
    content = content[:idx2] + content[end_idx:]

with open('lib/resilience/chaos/chaos_test.go', 'w') as f:
    f.write(content)
