with open('srcs/server/orchestration/cached_minimax_client_test.go', 'r') as f:
    content = f.read()
import re
content = re.sub(r'db\.NewDBProvider\(.*?\)', 'db.NewTestProvider(t)', content)

# Check for variables
lines = content.split('\n')
for i, line in enumerate(lines):
    if 'pool, cleanup := db.NewTestProvider(t)' in line:
        lines[i] = '\tprov := db.NewTestProvider(t)'
    elif 'pool := db.NewTestProvider(t)' in line:
        lines[i] = '\tprov := db.NewTestProvider(t)'
    elif 'defer cleanup()' in line:
        lines[i] = '\t// defer cleanup'
    elif 'defer pool.Close()' in line:
        lines[i] = '\t// defer pool.Close()'
    elif '_, err = prov.Exec(' in line:
        # Define err if it's the first occurrence
        lines[i] = '\t_, err := prov.Exec(context.Background(), `'
        for j in range(i+1, len(lines)):
            if '_, err := prov.Exec(' in lines[j] or '_, err = prov.Exec(' in lines[j]:
                 lines[j] = '\t_, err = prov.Exec(context.Background(), `'
        break
content = '\n'.join(lines)

with open('srcs/server/orchestration/cached_minimax_client_test.go', 'w') as f:
    f.write(content)
