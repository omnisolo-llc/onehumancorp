with open('srcs/server/orchestration/cached_minimax_client_test.go', 'r') as f:
    c = f.read()

c = c.replace('prov, err := db.NewTestProvider(t), "")', 'prov := db.NewTestProvider(t)')

# Also remove err checks right after
lines = c.split('\n')
for i, line in enumerate(lines):
    if 'prov := db.NewTestProvider(t)' in line:
        if i+1 < len(lines) and 'if err != nil {' in lines[i+1]:
            lines[i+1] = '\t// if err != nil {'
            lines[i+2] = '\t// \tt.Fatalf("failed to create db provider: %v", err)'
            lines[i+3] = '\t// }'

c = '\n'.join(lines)
with open('srcs/server/orchestration/cached_minimax_client_test.go', 'w') as f:
    f.write(c)
