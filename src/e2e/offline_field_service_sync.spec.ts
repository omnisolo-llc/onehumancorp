import { test, expect } from './fixtures';
import { seedRoutingJobs } from './support/field_routing_fixture';
import { e2eDbQuery } from './db_utils';

test.describe('Offline Field Service Sync', () => {
  test('preserves offline pending status and confirms the exact CAS request after reconnecting', async ({ page, context, seedData }) => {
    const [jobId] = await seedRoutingJobs(seedData.tenant.id, seedData.customer.id);
    const routes = page.waitForResponse(response => response.url().includes('/field-service-routing/routes/today'));
    await page.goto('/field-service-route.html');
    const snapshot = await (await routes).json() as { routes: { jobs: { id: string; updated_at: string }[] }[] };
    const observed = snapshot.routes.flatMap(route => route.jobs).find(job => job.id === jobId)!.updated_at;
    const card = page.getByTestId(`job-card-${jobId}`);
    await expect(card).toBeVisible();
    await context.setOffline(true);
    await expect(page.locator('#network-status-text')).toHaveText(/Working Offline/);
    await card.getByRole('button', { name: 'Start Travel', exact: true }).click();
    await expect(card.getByRole('button', { name: 'Arrived On-Site', exact: true })).toBeVisible();
    await expect(card.getByTestId(`job-receipt-${jobId}`)).toHaveText('Saved locally; confirmation pending.');
    expect((await e2eDbQuery('SELECT status FROM job_locations WHERE id = $1 AND tenant_id = $2', [jobId, seedData.tenant.id]))[0].status).toBe('pending');
    const responsePromise = page.waitForResponse(response => response.url().includes(`/jobs/${jobId}/status`) && response.request().method() === 'POST');
    await context.setOffline(false);
    const response = await responsePromise;
    expect(response.status()).toBe(200);
    expect(response.request().postDataJSON()).toEqual({ status: 'en_route', expected_updated_at: observed });
    expect(response.request().headers()['idempotency-key']).toBeTruthy();
    expect(response.request().headers()['x-ohc-expected-tenant']).toBe(seedData.tenant.id);
    expect(await response.json()).toMatchObject({ success: true, error: null, id: jobId, status: 'en_route' });
    await expect(card.getByTestId(`job-receipt-${jobId}`)).toHaveText('Confirmed by server.');
    await expect(page.locator('#network-status-indicator')).toBeHidden();
    await page.reload();
    await expect(card.getByRole('button', { name: 'Arrived On-Site', exact: true })).toBeVisible();
    expect((await e2eDbQuery('SELECT status FROM job_locations WHERE id = $1 AND tenant_id = $2', [jobId, seedData.tenant.id]))[0].status).toBe('en_route');
  });

  test('a real concurrent version change blocks the old offline intent until the owner discards and reviews it', async ({ page, context, seedData }) => {
    const [jobId] = await seedRoutingJobs(seedData.tenant.id, seedData.customer.id);
    await page.goto('/field-service-route.html');
    const card = page.getByTestId(`job-card-${jobId}`);
    await expect(card.getByRole('button', { name: 'Start Travel', exact: true })).toBeVisible();
    await context.setOffline(true);
    await card.getByRole('button', { name: 'Start Travel', exact: true }).click();
    await expect(card.getByTestId(`job-receipt-${jobId}`)).toHaveText('Saved locally; confirmation pending.');
    await e2eDbQuery("UPDATE job_locations SET updated_at = updated_at + INTERVAL '1 second' WHERE id = $1 AND tenant_id = $2", [jobId, seedData.tenant.id]);
    const response = page.waitForResponse(result => result.url().includes(`/jobs/${jobId}/status`) && result.request().method() === 'POST');
    await context.setOffline(false);
    expect((await response).status()).toBe(409);
    await expect(card.getByTestId(`job-receipt-${jobId}`)).toContainText('Change not saved (blocked)');
    expect((await e2eDbQuery('SELECT status FROM job_locations WHERE id = $1 AND tenant_id = $2', [jobId, seedData.tenant.id]))[0].status).toBe('pending');
    await card.getByRole('button', { name: 'Discard blocked changes and reload', exact: true }).click();
    await expect(card.getByRole('button', { name: 'Start Travel', exact: true })).toBeEnabled();
    await card.getByRole('button', { name: 'Start Travel', exact: true }).click();
    await expect(card.getByTestId(`job-receipt-${jobId}`)).toHaveText('Confirmed by server.');
  });
});
