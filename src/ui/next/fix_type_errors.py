import re

def fix_file(filename, replacements):
    with open(filename, 'r') as f:
        content = f.read()
    for old, new in replacements:
        content = content.replace(old, new)
    with open(filename, 'w') as f:
        f.write(content)

fix_file('src/app/bio/[tenant]/page.tsx', [
    ('<Footer theme={theme} tenantId={tenantId} />', '<Footer theme={theme as any} tenantId={tenantId} />')
])

fix_file('src/app/share-cards/page.tsx', [
    ('<Footer theme={theme} tenantId={shareLink.split("=")[1] || "my-store"} />', '<Footer theme={theme as any} tenantId={shareLink.split("=")[1] || "my-store"} />')
])

fix_file('src/app/wrapped/page.tsx', [
    ('<Footer theme="gradient" tenantId={tenant} />', '<Footer theme="gradient" tenantId={tenantId} />')
])

# Also need to fix imports for Footer if missing
def ensure_import(filename):
    with open(filename, 'r') as f:
        content = f.read()
    if 'import { Footer }' not in content:
        content = "import { Footer } from '../../components/Footer';\n" + content
    with open(filename, 'w') as f:
        f.write(content)

ensure_import('src/app/share-cards/page.tsx')
ensure_import('src/app/wrapped/page.tsx')
ensure_import('src/app/social-proof-nudge/page.tsx')
ensure_import('src/app/milestones/page.tsx')
