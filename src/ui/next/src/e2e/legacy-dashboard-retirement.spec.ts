import { test, expect } from '../../../../e2e/fixtures';
import { db } from '../../../../e2e/db_utils';
import { authenticateRequest } from '../../../../e2e/authenticate';
import { seedFeedItem } from '../../../../e2e/feed-fixtures';
import { seedDashboardAuditOwner } from '../../../../e2e/support/dashboard_audit_fixture';

// Explicit cases complement filesystem discovery: a new dashboard alias must
// be classified, rather than silently disappearing from retirement coverage.
const aliases = ['/api/ui/dashboard.html', '/api/v1/ui/dashboard.html'] as const;
const query = '?filter=first&filter=second&opaque=%e2%9c%93+%20';
for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
  test.describe(`API dashboard retirement at ${viewport.width}px`, () => {
    test.use({ viewport });
    for (const alias of aliases) {
      test(`${alias} preserves navigation and records an actual owner decision`, async ({ page, baseURL }, testInfo) => {
        if (!baseURL) throw new Error('An isolated real-stack fixture URL is required');
        const owner = await seedDashboardAuditOwner(baseURL);
        const origin = new URL(baseURL).origin;
        await authenticateRequest(page.request, { username: owner.email, password: owner.password, organizationId: owner.tenantId }, origin);
        await page.context().addInitScript(tenant => {
          localStorage.setItem('tenant_id', tenant); localStorage.setItem('tenant', tenant); localStorage.setItem('business_display_name', tenant);
        }, owner.tenantId);
        const draft = 'Review this recorded proposal; do not claim provider execution.';
        const id = await seedFeedItem(page, { event_source: 'operations', context_payload: { description: `Dashboard alias ${testInfo.testId}` }, proposed_action: { message: draft } }, owner.tenantId, { requestOrigin: origin });
        const missingAssets: string[] = [];
        page.on('response', response => { if (response.status() >= 400 && ['script', 'stylesheet'].includes(response.request().resourceType())) missingAssets.push(response.url()); });
        page.on('requestfailed', request => { if (['script', 'stylesheet'].includes(request.resourceType())) missingAssets.push(request.url()); });
        for (const method of ['GET', 'HEAD']) {
          const response = await page.request.fetch(alias + query, { method, maxRedirects: 0 });
          expect(response.status()).toBe(307);
          expect(response.headers().location).toBe(origin + '/dashboard' + query);
          expect(response.headers()['cache-control']).toContain('no-store');
          await response.dispose();
        }
        await page.goto(alias + query + '#compatibility', { waitUntil: 'domcontentloaded' });
        await expect(page).toHaveURL(origin + '/dashboard' + query + '#compatibility');
        const card = page.getByTestId(`triage-card-${id}`);
        await expect(card).toContainText(`Dashboard alias ${testInfo.testId}`);
        await expect(page.getByRole('link', { name: 'Create Agent Card', exact: true })).toHaveAttribute('href', '/agent-card.html');
        await expect(page.getByRole('link', { name: 'Certificate Generator', exact: true })).toHaveAttribute('href', '/viral-certificate-generator.html');
        const [response] = await Promise.all([
          page.waitForResponse(value => new URL(value.url()).pathname === `/api/v1/agent-feed/${id}` && value.request().method() === 'PUT'),
          card.getByTestId('feed-approve-btn').click(),
        ]);
        expect(response.status()).toBe(200);
        expect(await response.json()).toMatchObject({ id, tenant_id: owner.tenantId, lifecycle_state: 'APPROVED', decision_recorded: true });
        expect(await db.query('SELECT lifecycle_state FROM agent_feed_items WHERE id=$1 AND tenant_id=$2', [id, owner.tenantId])).toEqual([{ lifecycle_state: 'APPROVED' }]);
        await expect(page.getByText('Approval recorded. Execution or delivery is not verified by this decision.', { exact: true })).toBeVisible();
        await page.reload(); await expect(card).toBeHidden();
        // Back/Forward retains the canonical URL, ordered query, and fragment.
        await page.goto('/help'); await page.goBack();
        await expect(page).toHaveURL(origin + '/dashboard' + query + '#compatibility');
        await page.goForward(); await expect(page).toHaveURL(origin + '/help');
        expect(missingAssets).toEqual([]);
      });
    }
  });
}
for (const alias of aliases) {
  test(`anonymous ${alias} remains protected`, async ({ anonymousPage }) => {
    const response = await anonymousPage.request.get(alias + query, { maxRedirects: 0 });
    expect(response.status()).toBe(401); expect(response.headers()['cache-control']).toContain('no-store');
  });
}
