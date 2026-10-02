from pathlib import Path
import re

root = Path(__file__).resolve().parents[2]
server = (root / 'src/server/lib.rs').read_text()
route = re.search(r'\.route\(\s*"/api/v1/ledger/entries",(.*?)\n\s*\.route\(', server, re.S)
if not route or 'get(api::payment_ledger::get_entries)' not in route.group(1):
    raise SystemExit('Ledger GET is not mounted to the tested production handler')
if '::server_auth::strict_bearer_auth_middleware' not in route.group(1):
    raise SystemExit('Ledger GET no longer uses the tested bearer authentication')
bff = (root / 'src/ui/next/src/app/api/v1/[...path]/route.ts').read_text()
if 'export const GET = proxy;' not in bff or 'proxyBackendRequest(request, path' not in bff:
    raise SystemExit('The unmatched ledger GET is not routed through the authenticated BFF')
print('Production ledger handler, bearer middleware and generic BFF wiring verified')
