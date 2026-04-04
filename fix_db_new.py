with open('srcs/server/orchestration/autodream_kairos_test.go', 'r') as f:
    content = f.read()
# Replace db.NewTestProvider(t) with NewTestProvider(t) because I already imported it into the same package.
content = content.replace('db.NewTestProvider(t)', 'NewTestProvider(t)')
with open('srcs/server/orchestration/autodream_kairos_test.go', 'w') as f:
    f.write(content)
