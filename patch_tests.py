with open('srcs/server/orchestration/cached_minimax_client_test.go', 'r') as f:
    content = f.read()

import re
lines = content.split('\n')
for i, line in enumerate(lines):
    if 'pool, cleanup := db.NewTestProvider(ctx, "sqlite://:memory:")' in line or 'pool, cleanup := db.NewTestProvider(ctx' in line:
        lines[i] = '\tprov := db.NewTestProvider(t)'
    elif 'defer cleanup()' in line:
        lines[i] = '\t// defer prov.Close()'
    elif '_, err = prov.Exec(context.Background(), `' in line:
        # Check if err is defined in the block before
        if i > 5 and 'prov := db.NewTestProvider(t)' in '\n'.join(lines[i-5:i]):
             lines[i] = '\t_, err := prov.Exec(context.Background(), `'
        else:
             lines[i] = '\t_, err = prov.Exec(context.Background(), `'

content = '\n'.join(lines)
with open('srcs/server/orchestration/cached_minimax_client_test.go', 'w') as f:
    f.write(content)


with open('srcs/server/orchestration/ultraplan_test.go', 'r') as f:
    content = f.read()
content = content.replace('upm.CreatePlan(ctx, "mission1")', 'upm.CreatePlan(ctx, "mission1", map[string]interface{}{})')
content = content.replace('upm.CreatePlan(ctx, "m-123")', 'upm.CreatePlan(ctx, "m-123", map[string]interface{}{})')
with open('srcs/server/orchestration/ultraplan_test.go', 'w') as f:
    f.write(content)
