with open('srcs/server/db/provider.go', 'r') as f:
    lines = f.readlines()

new_lines = []
for line in lines:
    if "Ping(ctx context.Context) error" in line:
        if any("Ping(ctx context.Context) error" in l for l in new_lines):
            continue
    new_lines.append(line)

with open('srcs/server/db/provider.go', 'w') as f:
    f.writelines(new_lines)


with open('srcs/server/db/postgres_provider.go', 'r') as f:
    lines = f.readlines()

new_lines = []
skip = False
for line in lines:
    if "func (p *PgProvider) Ping(ctx context.Context) error {" in line:
        if any("func (p *PgProvider) Ping(ctx context.Context) error {" in l for l in new_lines):
            skip = True
    if skip:
        if "}" in line:
            skip = False
        continue
    new_lines.append(line)

with open('srcs/server/db/postgres_provider.go', 'w') as f:
    f.writelines(new_lines)


with open('srcs/server/db/sqlite_provider.go', 'r') as f:
    lines = f.readlines()

new_lines = []
skip = False
for line in lines:
    if "func (p *SqliteProvider) Ping(ctx context.Context) error {" in line:
        if any("func (p *SqliteProvider) Ping(ctx context.Context) error {" in l for l in new_lines):
            skip = True
    if skip:
        if "}" in line:
            skip = False
        continue
    new_lines.append(line)

with open('srcs/server/db/sqlite_provider.go', 'w') as f:
    f.writelines(new_lines)
