"""Compile exact public registry metadata and real service response serialization.
Provider initialization is outside this serializer contract; its metadata and
handshake are exercised separately through the actual NATS provider module.
"""
from pathlib import Path
import hashlib
import json
HERE=Path(__file__).resolve().parent; ROOT=HERE.parents[1]
registry=(ROOT/'src/server/integrations/registry.rs').read_text()
service=(ROOT/'src/server/services/integration/service.rs').read_text()
start=registry.index('    pub fn instances(')
end=registry.index('    pub fn connect(',start)
reads=registry[start:end]
start=end
end=registry.index('    pub fn disconnect(',start)
connect=registry[start:end]
stop='        insts.insert(integration_id.to_string(), inst.clone());'
assert connect.count(stop)==1 and connect.count('let inst =')==1
assert connect.rstrip().endswith('Ok(inst)\n    }'), 'Public return must remain the actual constructed instance'
construction=connect[:connect.index(stop)+len(stop)]+'\n        Ok(inst)\n    }\n'
start=service.index('    async fn get_integrations(')
end=service.index('    async fn disconnect_integration(',start)
responses=service[start:end]
lines=['#![allow(dead_code)]','extern crate self as server_integrations_core;',
       'include!('+json.dumps(str(ROOT/'src/server/integrations/core/mod.rs'))+');',
       '#[path='+json.dumps(str(ROOT/'src/server/integrations/registry_config.rs'))+']mod registry_config;',
       'pub mod integrations {',
       '#[path='+json.dumps(str(ROOT/'src/server/integrations/catalog.rs'))+']pub mod catalog;',
       '#[path='+json.dumps(str(ROOT/'src/server/integrations/nats/mod.rs'))+']pub mod nats;',
       'pub mod registry { use std::sync::RwLock;use crate::registry_config::{validate_registry_connection,RegistryConnectionInput};',
       '#[derive(Default)]pub struct IntegrationsRegistry {instances:RwLock<std::collections::HashMap<String,server_omnisolo::orchestration::IntegrationInstance>>}',
       'impl IntegrationsRegistry {',reads,construction,'}}}',
       'use server_omnisolo::orchestration::*;use tonic::{Request,Response,Status};',
       'struct MyIntegrationService {registry:std::sync::Arc<integrations::registry::IntegrationsRegistry>}',
       'impl MyIntegrationService {',responses,'}',
       '#[cfg(test)]#[path="test.rs"]mod tests;']
(HERE/'generated.rs').write_text('\n'.join(lines)+'\n')
inputs=[ROOT/p for p in ['Cargo.toml','Cargo.lock','docs/development/integration-configuration.md','scripts/integration-registry-credentials.test.mjs','src/server/integrations/core/mod.rs','src/server/integrations/registry_config.rs','src/server/integrations/catalog.rs','src/server/integrations/registry.rs','src/server/services/integration/service.rs','src/server/lib.rs','.github/workflows/ci.yml','scripts/focused_ci_gate.py','scripts/test_focused_ci_gate.py']]
for directory in ['src/server/integrations/nats','src/server/omnisolo','src/proto']:
 inputs += [p for p in (ROOT/directory).rglob('*') if p.is_file() and p.suffix in {'.rs','.toml','.proto'}]
inputs += [p for p in HERE.iterdir() if p.is_file() and p.name not in {'Cargo.lock','source-manifest.json'}]
(HERE/'source-manifest.json').write_text(json.dumps({str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(set(inputs))},indent=2)+'\n')
print('Actual registry construction/list and service response bodies fingerprinted; provider modules imported directly')
