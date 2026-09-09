import { readFileSync } from 'node:fs';
import path from 'node:path';
import { describe, expect, it } from 'vitest';

describe('Playwright authentication source contract', () => {
  const fixturesPath = path.resolve(process.cwd(), '../../e2e/fixtures.ts');

  it('creates one storage state during setup instead of logging in for every page fixture', () => {
    const config = readFileSync(
      path.resolve(process.cwd(), 'playwright.config.ts'),
      'utf8',
    );
    const fixtures = readFileSync(fixturesPath, 'utf8');
    const pageFixture = fixtures.match(
      /page:\s*async\s*\(\{ page[^]*?\n\s*\},\n\s*memberPage:/,
    )?.[0];

    expect(config).toContain('globalSetup:');
    expect(config).toContain('storageState');
    expect(config).toContain("baseURL: process.env.PLAYWRIGHT_BASE_URL ?? 'http://127.0.0.1:3000'");
    expect(pageFixture).toBeDefined();
    expect(pageFixture).not.toContain('loginAs(page');
  });

  it('provides an anonymous context without deriving the UI origin from BASE_URL', () => {
    const fixtures = readFileSync(fixturesPath, 'utf8');

    expect(fixtures).toContain('anonymousPage: Page');
    expect(fixtures).toMatch(/storageState:\s*\{\s*cookies:\s*\[\],\s*origins:\s*\[\]\s*\}/);
    expect(fixtures).toMatch(/anonymousPage:\s*async\s*\(\{[^}]*baseURL/);
    expect(fixtures).not.toContain('process.env.BASE_URL');
  });

  it.each([
    'dashboard-triage-edit.mock-contract.ts',
    'store-wrapped.spec.ts',
    'subscription_replenishment_feed.spec.ts',
    'hyperlocal-lead-gen.spec.ts',
  ])('does not repeat the default admin login in %s', (file) => {
    const source = readFileSync(path.resolve(process.cwd(), 'src/e2e', file), 'utf8');

    expect(source).not.toMatch(/goto\(['"]\/login['"]\)/);
    expect(source).not.toMatch(/adminPage\(/);
  });

  it('uses an anonymous page for the explicit alternate-user login flow', () => {
    const file = 'subscription_churn_retention.spec.ts';
    const source = readFileSync(path.resolve(process.cwd(), 'src/e2e', file), 'utf8');
    const seed = readFileSync(path.resolve(process.cwd(), '../../e2e/e2e-seed.sql'), 'utf8');

    expect(source).toContain('anonymousPage');
    expect(source).toContain('leo@example.com');
    expect(seed).toContain("'leo@example.com'");
    expect(seed).toContain("'e2e-feed-churn'");
  });
});
