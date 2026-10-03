// Real database fixture generation only. Never import this from production code.
const { createHash } = require('node:crypto');

function createOwnedAuditSeed(source, namespace) {
  if (!/^audit-[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}$/.test(namespace)) {
    throw new Error('A fresh audit UUID namespace is required');
  }
  const sections = [...source.matchAll(/-- audit-fixture-data:start\n([\s\S]*?)-- audit-fixture-data:end/g)];
  if (sections.length !== 2) throw new Error('Canonical audit fixture data sections are missing');
  const data = sections.map(([, sql]) => sql).join('\n');
  if (/^\s*(?:ALTER|CREATE|DROP|DO|BEGIN|COMMIT)\b/im.test(data)) throw new Error('Runner-only schema operations are forbidden in audit fixture data');
  const outside = source.replace(/-- audit-fixture-data:start\n[\s\S]*?-- audit-fixture-data:end/g, '').replace(/\$\$[\s\S]*?\$\$/g, '');
  if (/^\s*(?:INSERT INTO|UPDATE|DELETE FROM)\b/im.test(outside)) throw new Error('Canonical fixture data exists outside the owned sections');
  const sourceStatementCount = [...data.matchAll(/^\s*(?:INSERT INTO|UPDATE|DELETE FROM)\b/gm)].length;
  const canonicalIds = {};
  const uuid = (original) => {
    const hex = createHash('sha256').update(`${namespace}\0${original}`).digest('hex');
    const generated = `${hex.slice(0, 8)}-${hex.slice(8, 12)}-4${hex.slice(13, 16)}-a${hex.slice(17, 20)}-${hex.slice(20, 32)}`;
    canonicalIds[generated] = original;
    return generated;
  };
  // Rewrite both SQL values and nested JSON references from the same canonical
  // graph. No shared seeded ID, account email or global milestone key survives.
  const sql = data
    .replace(/\b[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}\b/g, uuid)
    .replace(/\b(?:bm-(?:e2e|default)-[a-z0-9-]+|opp-test-[a-z0-9-]+|e2e-[a-z0-9-]+)\b/g, value => `${namespace}-${value}`)
    .replace(/\b[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}\b/gi, value => `${namespace}-${value}`)
    .replace(/'DEFAULT'/g, `'${namespace}-default'`)
    // An unexpected natural-key collision must roll back, never update a
    // previous case or the suite's shared canonical records.
    .replace(/\nON CONFLICT[\s\S]*?;/g, ';');
  return { namespace, sql, canonicalIds, sourceDigest: createHash('sha256').update(source).digest('hex'), sourceStatementCount, tenantId: `${namespace}-e2e-tenant`,
    userId: `${namespace}-e2e-admin-user`, email: `${namespace}-test@example.com`, password: 'password123' };
}

function assertSameClickInventory(baseline, current) {
  if (new Set(baseline).size !== baseline.length || new Set(current).size !== current.length) throw new Error('Dashboard baseline contains duplicate target keys');
  const missing = baseline.filter(key => !current.includes(key));
  const unexpected = current.filter(key => !baseline.includes(key));
  if (missing.length || unexpected.length) throw new Error(`Dashboard baseline changed before a click; missing=${JSON.stringify(missing)}; unexpected=${JSON.stringify(unexpected)}`);
}

module.exports = { createOwnedAuditSeed, assertSameClickInventory };
