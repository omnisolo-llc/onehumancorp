import { randomUUID } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import type { Browser, Page, Request } from '@playwright/test';
import { createOwnedAuditSeed } from '../../../scripts/ui-audit-fixture.cjs';
import { e2eDbTransaction } from '../db_utils';
import { authenticateRequest } from '../authenticate';
import { createAuditNavigation, type AuditNavigationReceipt } from './ui_audit_navigation';

export const isolatedClickAuditRoutes = new Set([
  '/', '/dashboard', '/unified-feed', '/dashboard/unified-feed', '/feed', '/action-center', '/builder', '/website-builder',
]);

export function clickAuditStates(route: string): string[] {
  return route === '/builder' || route === '/website-builder' ? ['entry', 'started-draft'] : ['entry'];
}

// Both preparations are local wizard choices, never a dispatched business
// action. Recreate them only in a newly seeded owner/context, before observation.
export async function prepareClickAuditState(page: Page, route: string, state: string) {
  if (state === 'entry') return;
  if (state !== 'started-draft') throw new Error(`Unknown click audit state: ${state}`);
  if (route === '/website-builder') {
    await page.getByRole('button', { name: 'Start My Business', exact: true }).click();
    await page.getByRole('button', { name: 'Online Store', exact: true }).waitFor({ state: 'visible' });
  } else if (route === '/builder') {
    await page.getByRole('button', { name: /Selling Products/ }).click();
    await page.getByRole('button', { name: 'Next: Choose Vibe', exact: true }).waitFor({ state: 'visible' });
  } else throw new Error(`No draft preparation exists for ${route}`);
}

const initialRouteReads: Record<string, string[]> = {
  '/': ['/api/v1/ui/dashboard/unified-feed'],
  '/dashboard': ['/api/v1/ui/dashboard/unified-feed'],
  '/unified-feed': ['/api/v1/agent-feed'],
  '/dashboard/unified-feed': ['/api/v1/agent-feed'],
  '/feed': ['/api/v1/agent-feed'],
  '/action-center': ['/api/v1/agents/approvals'],
  '/builder': ['/api/v1/auth/session-identity'],
  '/website-builder': ['/api/v1/onboarding/draft', '/api/v1/onboarding/state'],
};

const canonicalSeed = () => readFileSync(path.resolve(__dirname, '../e2e-seed.sql'), 'utf8');

export async function seedDashboardAuditOwner(baseURL: string) {
  const base = new URL(baseURL);
  if (!['http:', 'https:'].includes(base.protocol) || !['127.0.0.1', 'localhost', '[::1]'].includes(base.hostname)
      || base.username || base.password) throw new Error('Dashboard fixtures require the isolated local app');
  const seed = createOwnedAuditSeed(canonicalSeed(), `audit-${randomUUID()}`);
  // e2eDbTransaction validates the current native runner's private proof file,
  // exact Docker identity, labels and loopback PostgreSQL port before connecting.
  await e2eDbTransaction(async query => {
    await query(seed.sql);
    const owners = await query(`SELECT u.id, u.tenant_id FROM users u
      JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id
      WHERE u.id=$1 AND u.username=$2 AND u.tenant_id=$3 AND u.active=true
        AND u.roles=ARRAY['ADMIN']::text[] AND r.role_name='ADMIN'`, [seed.userId, seed.email, seed.tenantId]);
    if (owners.length !== 1) throw new Error('Case-owned normalized admin identity was not persisted');
    const graph = await query(`SELECT
      (SELECT count(*)::int FROM tenants WHERE id LIKE $1) AS tenants,
      (SELECT count(*)::int FROM agent_feed_items WHERE tenant_id=$2) AS feed,
      (SELECT count(*)::int FROM agent_approvals WHERE tenant_id=$2) AS approvals,
      (SELECT count(*)::int FROM omni_inbox_messages WHERE tenant_id=$2) AS inbox,
      (SELECT count(*)::int FROM products WHERE tenant_id=$2) AS products,
      (SELECT count(*)::int FROM opportunities WHERE tenant_id=$2) AS opportunities`, [`${seed.namespace}-%`, seed.tenantId]);
    if (JSON.stringify(graph[0]) !== JSON.stringify({ tenants: 5, feed: 8, approvals: 2, inbox: 1, products: 2, opportunities: 2 })) {
      throw new Error(`Canonical case-owned dashboard graph was not persisted: ${JSON.stringify(graph[0])}`);
    }
  });
  return seed;
}

/** A fresh signed owner, durable dataset, cookie jar and storage for each click. */
export async function createDashboardAuditCase(browser: Browser, baseURL: string, viewport: { width: number; height: number } | null, videoDirectory?: string) {
  const actor = await seedDashboardAuditOwner(baseURL);
  const context = await browser.newContext({ baseURL, ...(viewport ? { viewport } : {}), ...(videoDirectory ? { recordVideo: { dir: videoDirectory } } : {}) });
  const pending = new Set<Request>();
  context.on('request', request => {
    if (new URL(request.url()).origin === new URL(baseURL).origin && ['fetch', 'xhr'].includes(request.resourceType())) pending.add(request);
  });
  context.on('requestfinished', request => pending.delete(request));
  context.on('requestfailed', request => pending.delete(request));
  try {
    await context.addInitScript(tenant => {
      // about:blank has an opaque origin before the first application navigation.
      if (location.protocol === 'http:' || location.protocol === 'https:') {
        localStorage.setItem('tenant_id', tenant);
        localStorage.setItem('tenant', tenant);
        localStorage.setItem('business_display_name', tenant);
      }
    }, actor.tenantId);
    const page = await context.newPage();
    const navigate = createAuditNavigation(baseURL, async target => {
      await authenticateRequest(target.request, { username: actor.email, password: actor.password, organizationId: actor.tenantId }, new URL(baseURL).origin);
      const response = await target.request.get('/api/v1/auth/session-identity');
      if (response.status() !== 200) throw new Error('Dashboard audit identity was not verified');
      const identity = await response.json();
      if (identity.userId !== actor.userId || identity.tenantId !== actor.tenantId) throw new Error('Dashboard audit reached a different owner');
    });
    return { actor, page, close: () => context.close(), navigate: async (route = '/dashboard'): Promise<AuditNavigationReceipt> => {
      if (!isolatedClickAuditRoutes.has(route)) throw new Error(`No isolated click fixture exists for ${route}`);
      const initialReads = initialRouteReads[route].map(path => page.waitForResponse(response =>
        new URL(response.url()).pathname === path && response.request().method() === 'GET'));
      // Complete the real initial reads before discovery; no API substitution.
      const [receipt, responses] = await Promise.all([navigate(page, route), Promise.all(initialReads)]);
      for (const response of responses) {
        if (response.status() !== 200) throw new Error(`${route} baseline read failed: HTTP ${response.status()}`);
        await response.finished();
      }
      if (route === '/' || route === '/dashboard') {
        await page.getByText('Loading business metrics…', { exact: true }).waitFor({ state: 'hidden' });
        await page.getByText('Loading Agent Proposals...', { exact: true }).waitFor({ state: 'hidden' });
      } else if (route === '/unified-feed' || route === '/dashboard/unified-feed') {
        await page.getByText('Loading feed...', { exact: true }).waitFor({ state: 'hidden' });
      } else if (route === '/feed') {
        await page.getByText('Checking your feed...', { exact: true }).waitFor({ state: 'hidden' });
      }
      const deadline = Date.now() + 5000;
      while (true) {
        // Rendering after completed responses is scheduled in the browser realm.
        await page.evaluate(() => new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
        if (pending.size === 0) break;
        if (Date.now() >= deadline) throw new Error(`${route} baseline reads did not settle before discovery`);
        await page.waitForTimeout(25);
      }
      return receipt;
    } };
  } catch (error) {
    await context.close();
    throw error;
  }
}

export type DashboardAuditCase = Awaited<ReturnType<typeof createDashboardAuditCase>>;
