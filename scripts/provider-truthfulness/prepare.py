"""Fingerprint the complete production files compiled by this focused gate."""
import hashlib
import json
from pathlib import Path

here = Path(__file__).resolve().parent
root = here.parents[1]
paths = [
    'src/server/integrations/twilio/client.rs',
    'src/server/integrations/twilio/message_tests.rs',
    'src/server/integrations/zoom/client.rs',
    'src/server/integrations/daily/client.rs',
    'src/server/integrations/meeting_receipt_tests.rs',
    'src/server/integrations/cal_com/client.rs',
    'src/server/integrations/cal_com/client_test.rs',
    'src/server/api/integrations_settings.rs',
    'scripts/provider-truthfulness/Cargo.toml',
    'scripts/provider-truthfulness/lib.rs',
    'Cargo.lock',
]
manifest = {path: hashlib.sha256((root / path).read_bytes()).hexdigest() for path in paths}
(here / 'source-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
