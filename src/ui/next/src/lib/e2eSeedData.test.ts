import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { describe, expect, it } from 'vitest';

import { E2E_SEED_DATA } from './e2eSeedData';

describe('E2E seed fixture contract', () => {
  it('matches the tenant and customer loaded by the PostgreSQL seed', () => {
    const rootCandidate = path.resolve(process.cwd(), 'src/e2e/e2e-seed.sql');
    const seedPath = existsSync(rootCandidate)
      ? rootCandidate
      : path.resolve(process.cwd(), '../../e2e/e2e-seed.sql');
    const seedSql = readFileSync(seedPath, 'utf8');

    expect(seedSql).toContain(`'${E2E_SEED_DATA.tenant.id}'`);
    expect(seedSql).toContain(`'${E2E_SEED_DATA.customer.id}'`);
  });
});
