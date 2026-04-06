import re

with open('srcs/server/orchestration/service.go', 'r') as f:
    content = f.read()

# Fix the SetSIPDB signature that got mangled
content = re.sub(
    r'// Returns SetSIPDB\(sipDB\s+\*SIPDB\n\s+taskManager\s+\*TaskManager\)\.',
    '// Returns SetSIPDB(sipDB *SIPDB).',
    content
)

content = re.sub(
    r'func \(h \*Hub\) SetSIPDB\(sipDB\s+\*SIPDB\n\s+taskManager\s+\*TaskManager\) \{',
    'func (h *Hub) SetSIPDB(sipDB *SIPDB) {',
    content
)

with open('srcs/server/orchestration/service.go', 'w') as f:
    f.write(content)
