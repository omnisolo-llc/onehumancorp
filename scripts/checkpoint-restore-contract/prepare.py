"""Compile the complete checkpointer and exact protocol restore method, source-bound."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
ralph = (ROOT / 'src/agents/builtin/ralph_loop.rs').read_text()
progress = ralph[ralph.index('#[derive(Serialize'):ralph.index('/// The "Ralph Loop":')]
protocol = (ROOT / 'src/agents/builtin/agent_protocol.rs').read_text()
restore = protocol[protocol.index('    pub async fn restore_checkpoint('):protocol.index('    pub async fn download_artifact(')]
(HERE / 'generated.rs').write_text('''#![allow(dead_code)]
#[path="../../src/agents/builtin/checkpointer.rs"] pub mod checkpointer;
pub mod ralph_loop {
use serde::{Serialize, Deserialize};
''' + progress + '''
}
// Minimal receiver plumbing only: the restore method below is copied verbatim.
// This proves that method and the real stores, not full runtime/tenant admission.
struct Agent { checkpointer: Option<std::sync::Arc<dyn checkpointer::CheckpointSaver>> }
struct Core { agent: Agent }
struct Runner { core: Core }
struct AgentProtocolServer { runner: Runner }
impl AgentProtocolServer {
''' + restore + '''
}
#[cfg(test)] mod tests;
''')
inputs = ['.github/workflows/ci.yml', 'scripts/focused_ci_gate.py', 'scripts/test_focused_ci_gate.py', 'scripts/checkpoint-restore-contract/ci.test.py', 'src/agents/builtin/checkpointer.rs', 'src/agents/builtin/agent_protocol.rs',
          'src/agents/builtin/ralph_loop.rs', 'src/agents/builtin/agent.rs',
          'src/agents/builtin/gather_act_verify.rs', 'scripts/checkpoint-restore-contract/prepare.py',
          'scripts/checkpoint-restore-contract/tests.rs', 'scripts/checkpoint-restore-contract/Cargo.toml',
          'scripts/checkpoint-restore-contract/Cargo.lock']
manifest = {p: hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in inputs if (ROOT / p).exists()}
(HERE / 'source-manifest.json').write_text(json.dumps(manifest, indent=2, sort_keys=True) + '\n')
