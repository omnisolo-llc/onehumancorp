import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
const root = new URL('../', import.meta.url);
const read = path => readFile(new URL(path, root), 'utf8');

test('marketplace RPC connection failures cannot invent success or replay through a fallback', async () => {
  const source = await read('src/server/lib.rs');
  const handler = source.slice(source.indexOf('async fn proxy_agent_rpc_handler('), source.indexOf('fn strict_ui_claim_tenant('));
  const failed = handler.slice(handler.indexOf('let Ok(upstream) = request.send().await else'));
  assert.doesNotMatch(failed, /MP_CLIENT|MockMarketplaceProvider/);
  const firstFallback = failed.indexOf('FALLBACK_APP_SERVER');
  assert.ok(firstFallback > 0);
  const boundary = failed.slice(0, firstFallback);
  for (const method of ['am_publish_agent', 'am_search_agents', 'am_fetch_agent']) assert.ok(boundary.includes('"' + method + '"'));
  assert.match(boundary, /StatusCode::SERVICE_UNAVAILABLE/);
  assert.doesNotMatch(boundary, /publish_agent\(|fetch_agent\(|\.search\(/);
});
test('production marketplace constructors require configured providers instead of test catalogues', async () => {
  for (const path of ['src/agents/builtin/codex_runner.rs', 'src/server/api/agents/hire.rs']) {
    const source = await read(path);
    const production = source.split('#[cfg(test)]')[0];
    assert.doesNotMatch(production, /test_utils::MockMarketplaceProvider/);
  }
});
test('native publication dispatch keeps descriptor fields and rejects unsupported request fields', async () => {
  const source = await read('src/agents/builtin/codex_runner.rs');
  const start = source.indexOf('} else if req.method == "am_publish_agent"');
  const branch = source.slice(start, source.indexOf('let published =', start));
  assert.match(branch, /serde_json::from_value/);
  assert.doesNotMatch(branch, /get\("role"\)|http:\/\/localhost|version: "1\.0\.0"/);
  const descriptor = await read('src/agents/builtin/tools/marketplace.rs');
  assert.match(descriptor, /#\[serde\(deny_unknown_fields\)\]/);
});
test('marketplace tool publication halts on an unknown outcome instead of automatic retry', async () => {
  const source = await read('src/agents/builtin/tools/marketplace_tool.rs');
  const start = source.indexOf('} else if action == "publish"');
  const branch = source.slice(start, source.indexOf('        } else {', start));
  assert.match(branch, /validate_publication\(\)/);
  assert.match(branch, /Err\(e\) => Err\(ToolError::UserFixable\(e\)\)/);
  assert.doesNotMatch(branch, /ToolError::Transient/);
});
