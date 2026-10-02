from pathlib import Path
import hashlib, json
ROOT = Path(__file__).resolve().parents[2]
runner_path = ROOT / 'src/agents/builtin/codex_runner.rs'
hire_path = ROOT / 'src/server/api/agents/hire.rs'
runner = runner_path.read_text()
start = runner.index('        let marketplace_url =', runner.index('impl AppServer'))
end = runner.index('        Self {', start)
factory = runner[start:end]
hire = hire_path.read_text()
start = hire.index('#[derive(Deserialize, Debug)]\npub struct MarketplaceQuery')
listing = hire[start:]
header = '''use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use std::sync::Arc;
use crate::tools;
'''
source = header + '\n// Keep the production binding intact; its real caller builds Self afterward.\n#[allow(clippy::let_and_return)]\npub fn configured_app_marketplace() -> Arc<tools::marketplace::MarketplaceClient> {\n' + factory + '    marketplace\n}\n\n' + listing
(ROOT / 'scripts/marketplace-contract/src/generated.rs').write_text(source)
manifest = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in [runner_path, hire_path, ROOT/'src/agents/builtin/tools/marketplace.rs']}
manifest['generated_sha256'] = hashlib.sha256(source.encode()).hexdigest()
(ROOT / 'scripts/marketplace-contract/source-manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')

tools_path = ROOT / 'src/agents/builtin/tools/mod.rs'
tools = tools_path.read_text()
start = tools.index('/// A tool definition and executor')
end = tools.index('/// Shared todo list state', start)
base = 'use serde_json::Value;\nuse std::sync::Arc;\nuse crate::types::ToolError;\n' + tools[start:end]
(ROOT / 'scripts/marketplace-contract/src/generated-tool-base.rs').write_text(base)
for path in ['src/agents/builtin/tools/mod.rs', 'src/agents/builtin/tools/pydantic.rs', 'src/agents/builtin/tools/marketplace_tool.rs', 'src/agents/builtin/types.rs']:
    manifest[path] = hashlib.sha256((ROOT/path).read_bytes()).hexdigest()
manifest['generated_tool_base_sha256'] = hashlib.sha256(base.encode()).hexdigest()
(ROOT / 'scripts/marketplace-contract/source-manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')

rpc_types = runner[runner.index('#[derive(Serialize, Deserialize)]'):runner.index('pub struct AppServer')]
start = runner.index('} else if req.method == "am_publish_agent" {') + len('} else if req.method == "am_publish_agent" {')
end = runner.index('        } else if req.method == "ap_list_checkpoints"', start)
publication = runner[start:end]
rpc = 'use serde::{Deserialize, Serialize};\nuse std::sync::Arc;\n' + rpc_types
rpc += '\npub struct PublicationCall { pub marketplace: Arc<crate::marketplace::MarketplaceClient> }\nimpl PublicationCall {\n// Preserve the complete production dispatch branch, including its final return.\n#[allow(clippy::needless_return)]\npub async fn publish(&self, req: JsonRpcRequest) -> String {\n' + publication + '\n}\n}\n'
(ROOT / 'scripts/marketplace-contract/src/generated-publication.rs').write_text(rpc)
manifest['generated_publication_sha256'] = hashlib.sha256(rpc.encode()).hexdigest()
(ROOT / 'scripts/marketplace-contract/source-manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
