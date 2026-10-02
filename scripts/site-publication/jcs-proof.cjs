// Independent ECMAScript/RFC8785 witness for real PostgreSQL/Rust receipts.
if (Number(process.versions.node.split('.')[0]) !== 22) throw new Error('Publication proof requires pinned Node 22');
const canonicalize = require('canonicalize').default;
const { dirname, join } = require('node:path');
const { createHash } = require('node:crypto');
const { readFileSync } = require('node:fs');
if (JSON.parse(readFileSync(join(dirname(require.resolve('canonicalize')), '../package.json'), 'utf8')).version !== '5.1.0') {
  throw new Error('Publication proof requires canonicalize 5.1.0');
}
const snapshot = JSON.parse(readFileSync(0, 'utf8'));
const canonical = canonicalize(snapshot);
process.stdout.write(JSON.stringify({canonical, sha256: createHash('sha256').update(canonical).digest('hex')}));
