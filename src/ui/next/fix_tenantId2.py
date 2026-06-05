import re

with open('src/app/wrapped/page.tsx', 'r') as f:
    content = f.read()

content = content.replace('<Footer theme="gradient" tenantId={tenant} />', '<Footer theme="gradient" tenantId={typeof localStorage !== "undefined" ? localStorage.getItem("tenant") || "my-store" : "my-store"} />')

with open('src/app/wrapped/page.tsx', 'w') as f:
    f.write(content)
