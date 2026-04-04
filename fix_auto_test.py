with open('srcs/server/orchestration/autodream_kairos_test.go', 'r') as f:
    content = f.read()

content = content.replace('"github.com/onehumancorp/mono/srcs/server/db"\n', '')

with open('srcs/server/orchestration/autodream_kairos_test.go', 'w') as f:
    f.write(content)
