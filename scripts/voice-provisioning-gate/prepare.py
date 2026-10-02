"""Bind complete production voice routes, provider client, and storage implementations."""
from pathlib import Path
import hashlib
import json

here = Path(__file__).resolve().parent
root = here.parents[1]
source = (root / 'src/server/lib.rs').read_text()
assert '.merge(api::voice_provisioning::router(settings_store.clone()))' in source
assert '.route("/api/v1/settings/voice",' not in source
routes = '''    voice_provisioning::router_with_state(voice_provisioning::VoiceState {
        store: settings_store, provider: Arc::new(provider::TwilioProvider),
        standalone: _standalone, multitenant: !_standalone, configured: true,
        account_sid: "AC11111111111111111111111111111111".into(),
    })'''
common = (root / 'src/server/common/mod.rs').read_text()
begin = common.index('#[derive(Debug, Serialize, Deserialize, Clone)]')
finish = common.index('\n}', begin) + 2
claims = common[begin:finish]
template = (here / 'harness.rs.in').read_text()
generated = template.replace('@ROOT@', str(root)).replace('@CLAIMS@', claims).replace('@ROUTES@', routes)
(here / 'generated.rs').write_text(generated)
paths = ['src/server/lib.rs', 'src/server/settings.rs', 'src/server/common/mod.rs', 'src/server/utils/fs.rs', 'src/server/api/voice_provisioning.rs', 'src/server/api/mod.rs', 'src/server/integrations/twilio/client.rs', 'scripts/voice-provisioning-gate/harness.rs.in', 'scripts/voice-provisioning-gate/Cargo.toml', 'scripts/voice-provisioning-gate/prepare.py', 'scripts/voice-provisioning-gate/run.sh', 'scripts/voice-provisioning-gate/verify_lock.py']
manifest = {path: hashlib.sha256((root / path).read_bytes()).hexdigest() for path in paths}
manifest['state_adapter'] = hashlib.sha256(routes.encode()).hexdigest()
(here / 'source-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
