"""Import the entire real campaign service and repositories, including existing tests."""
from pathlib import Path
import hashlib
import json

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]

def module(path, name):
    return f'#[path={json.dumps(str(ROOT / path))}] pub mod {name};\n'

source = '#![allow(dead_code)]\n'
source += 'pub mod domain {pub mod repository {\n'
for name in ['models', 'campaign_repo', 'social_post_proposal_repo']:
    source += module(f'src/server/domain/repository/{name}.rs', name)
source += 'pub use social_post_proposal_repo::SocialPostProposalRepository;\n}}\n'
source += 'pub mod integrations {\n' + module('scripts/campaign-durability/registry_sentinel.rs', 'registry') + '}\n'
source += 'pub mod services {\n' + module('src/server/services/campaign/mod.rs', 'campaign') + '}\n'
(HERE / 'generated.rs').write_text(source)
inputs = [ROOT / p for p in ['Cargo.lock', 'src/server/lib.rs', 'src/server/domain/repository/models.rs', 'src/server/domain/repository/campaign_repo.rs', 'src/server/domain/repository/social_post_proposal_repo.rs', 'src/server/integrations/registry.rs', 'src/server/migrations/001_initial.sql', 'src/server/migrations/063_campaign_engine.sql']]
inputs += list((ROOT / 'src/server/services/campaign').glob('*.rs'))
inputs += list(HERE.glob('*.py')) + [HERE / 'Cargo.toml', HERE / 'registry_sentinel.rs', HERE / 'run.sh', HERE / 'README.md']
inputs += list((ROOT / 'src/server/auth').glob('*.rs')) + list((ROOT / 'src/server/omnisolo').glob('*.rs')) + list((ROOT / 'src/proto').glob('*.proto'))
(HERE / 'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))}, indent=2) + '\n')
print('Prepared whole-source campaign service/repository harness; provider calls are negative sentinels')
