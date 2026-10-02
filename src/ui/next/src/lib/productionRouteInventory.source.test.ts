import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { discoverApplicationRoutes } from '../e2e/production_route_inventory';

const roots: string[] = [];
function fixture() {
  const root = mkdtempSync(path.join(tmpdir(), 'ohc-route-inventory-'));
  roots.push(root);
  for (const route of ['bio/[tenant]', 'orders/[id]', 'help/[articleId]', 'dashboard']) {
    const directory = path.join(root, route);
    mkdirSync(directory, { recursive: true });
    writeFileSync(path.join(directory, 'page.tsx'), 'export default function Page() { return null; }');
  }
  return root;
}
afterEach(() => { for (const root of roots.splice(0)) rmSync(root, { recursive: true }); });

describe('smoke route inventory tenant binding', () => {
  it('preserves every default route when no fixture owner is supplied', () => {
    expect(discoverApplicationRoutes(fixture())).toEqual([
      '/bio/e2e-tenant', '/dashboard', '/help/getting-started-1', '/orders/e2e-route-record',
    ]);
  });
  it('substitutes only the verified fixture tenant without dropping routes', () => {
    expect(discoverApplicationRoutes(fixture(), { tenant: 'owned-profile' })).toEqual([
      '/bio/owned-profile', '/dashboard', '/help/getting-started-1', '/orders/e2e-route-record',
    ]);
  });
  it('keeps an opaque owner value in one URL path segment', () => {
    expect(discoverApplicationRoutes(fixture(), { tenant: 'owner/雪?x=1#fragment' })).toContain(
      '/bio/owner%2F%E9%9B%AA%3Fx%3D1%23fragment',
    );
  });
});
