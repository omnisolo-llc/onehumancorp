import re

with open('src/app/wrapped/page.tsx', 'r') as f:
    content = f.read()

content = content.replace('<Footer theme="gradient" tenantId={tenantId} />', '<Footer theme="gradient" tenantId={tenant} />')

with open('src/app/wrapped/page.tsx', 'w') as f:
    f.write(content)
