import type { Response } from '@playwright/test';
import { test, expect } from '../fixtures';
import { seedRoutingJobs } from '../support/field_routing_fixture';
import { e2eDbQuery } from '../db_utils';

test.describe('Field Service Routing Mobile App', () => {
  test('Carlos views today route and updates job status', async ({ page, seedData }) => {
    const [job1, job2] = await seedRoutingJobs(seedData.tenant.id, seedData.customer.id);
    // 1. Emulate a mobile device layout by changing viewport
    await page.setViewportSize({ width: 375, height: 667 });

    // 2. We use test which implies we are already logged in to the shell dashboard.
    // For this standalone html, we can navigate directly, but it relies on session cookie for auth.
    // The previous implementation explicitly skipped login which caused it to fail CI code review.
    // We already have auth from the fixture. We just navigate.
    const identityResponse = await page.request.get('/api/v1/auth/session-identity');
    expect(identityResponse.status()).toBe(200);
    const identity = await identityResponse.json();
    expect(identity.tenantId).toBe(seedData.tenant.id);
    const initialRoutes = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/field-service-routing/routes/today' && response.request().method() === 'GET',
    );
    await page.goto('/ui/field-service-route.html');
    const routesResponse = await initialRoutes;
    expect(routesResponse.status()).toBe(200);
    const routes: { routes: { jobs: { id: string; updated_at: string }[] }[] } = await routesResponse.json();
    const versions = new Map(routes.routes.flatMap(route => route.jobs).map(job => [job.id, job.updated_at]));
    const requestKeys = new Set<string>();
    const waitForStatus = (jobId: string, status: string) => page.waitForResponse(response =>
      new URL(response.url()).pathname === `/api/v1/field-service-routing/jobs/${encodeURIComponent(jobId)}/status`
      && response.request().method() === 'POST' && response.request().postDataJSON().status === status,
    );
    const verifyStatus = async (response: Response, jobId: string, status: string) => {
      expect(response.status()).toBe(200);
      const previousVersion = versions.get(jobId);
      expect(previousVersion).toEqual(expect.any(String));
      expect(response.request().postDataJSON()).toEqual({ status, expected_updated_at: previousVersion });
      const headers = await response.request().allHeaders();
      expect(headers['x-ohc-expected-user']).toBe(identity.userId);
      expect(headers['x-ohc-expected-tenant']).toBe(identity.tenantId);
      const key = headers['idempotency-key'];
      expect(key).toMatch(/^[0-9a-f-]{36}$/i);
      expect(requestKeys.has(key)).toBe(false);
      requestKeys.add(key);
      const receipt = await response.json();
      expect(receipt).toEqual({ success: true, error: null, id: jobId, status, updated_at: expect.any(String) });
      // PostgreSQL compares the exact timestamp, including submillisecond CAS precision.
      const saved = await e2eDbQuery(`SELECT status, updated_at = $3::timestamptz AS matches_receipt,
        updated_at > $4::timestamptz AS advanced FROM job_locations WHERE id = $1 AND tenant_id = $2`,
      [jobId, seedData.tenant.id, receipt.updated_at, previousVersion]);
      expect(saved).toEqual([{ status, matches_receipt: true, advanced: true }]);
      const committed = await e2eDbQuery(`SELECT actor_id, response FROM field_mutation_receipts
        WHERE tenant_id = $1 AND operation = 'routing_job' AND idempotency_key = $2`, [seedData.tenant.id, key]);
      expect(committed).toEqual([{ actor_id: identity.userId, response: receipt }]);
      const card = page.getByTestId(`job-card-${jobId}`);
      await expect(card.locator('.job-status')).toHaveText(status.replaceAll('_', ' '));
      await expect(card.getByTestId(`job-receipt-${jobId}`)).toHaveText('Confirmed by server.');
      versions.set(jobId, receipt.updated_at);
    };

    // 3. Verify page title and header
    await expect(page.locator('.header-title')).toHaveText("Today's Route");

    // 4. Wait for jobs to load from API
    await expect(page.locator('#loading-state')).toBeHidden({ timeout: 10000 });

    // Ensure we are displaying the seeded jobs from e2e-seed.sql
    const job1Card = page.locator(`[data-testid="job-card-${job1}"]`);
    const job2Card = page.locator(`[data-testid="job-card-${job2}"]`);

    await expect(job1Card).toBeVisible();
    await expect(job1Card.locator('.job-title')).toHaveText('Fix leaking sink');
    await expect(job1Card.locator('.job-status')).toHaveText('pending');

    await expect(job2Card).toBeVisible();

    // 5. CUJ Action: "Start Travel" (change status from pending -> en_route)
    const startTravelBtn = job1Card.locator(`[data-testid="btn-start-travel-${job1}"]`);
    await expect(startTravelBtn).toBeVisible();
    const travelResponse = waitForStatus(job1, 'en_route');
    await startTravelBtn.click();
    await verifyStatus(await travelResponse, job1, 'en_route');

    // 6. Verify status updated to 'en_route' and button changed to 'Arrived On-Site'
    await expect(job1Card.locator('.job-status')).toHaveText('en route', { timeout: 10000 });
    const arriveBtn = job1Card.locator(`[data-testid="btn-arrived-${job1}"]`);
    await expect(arriveBtn).toBeVisible();

    // 7. CUJ Action: "Arrived On-Site" (change status from en_route -> on_site)
    const arrivedResponse = waitForStatus(job1, 'on_site');
    await arriveBtn.click();
    await verifyStatus(await arrivedResponse, job1, 'on_site');

    // 8. Verify status updated to 'on_site'
    await expect(job1Card.locator('.job-status')).toHaveText('on site', { timeout: 10000 });

    // 9. CUJ Action: "Job Done" (change status from on_site -> done)
    const doneBtn = job1Card.locator(`[data-testid="btn-job-done-${job1}"]`);
    const doneResponse = waitForStatus(job1, 'done');
    await doneBtn.click();
    await verifyStatus(await doneResponse, job1, 'done');

    // 10. Verify status updated to 'done' and 'Tap to Pay' button is shown
    await expect(job1Card.locator('.job-status')).toHaveText('done', { timeout: 10000 });
    const payBtn = job1Card.locator('button', { hasText: 'Tap to Pay' });
    await expect(payBtn).toBeVisible();

    // Go offline
    await page.context().setOffline(true);
    await expect(page.locator('#network-status-text')).toHaveText(/Working Offline/);

    // CUJ Action: "Start Travel" (change status from pending -> en_route) on job2 while offline
    const startTravelBtn2 = job2Card.locator(`[data-testid="btn-start-travel-${job2}"]`);
    await expect(startTravelBtn2).toBeVisible();
    await startTravelBtn2.click();

    // Verify optimistic UI update while offline
    await expect(job2Card.locator('.job-status')).toHaveText('en route', { timeout: 10000 });

    // Check that network status indicator shows offline sync state
    const offlineIndicator = page.locator('#network-status-indicator');
    await expect(offlineIndicator).toBeVisible();
    await expect(offlineIndicator.locator('#network-status-text')).toHaveText('Working Offline - Changes Saved Locally; confirmation pending');

    await expect(job2Card.getByTestId(`job-receipt-${job2}`)).toHaveText('Saved locally; confirmation pending.');

    // The local optimistic status is not a persisted result while offline.
    expect((await e2eDbQuery('SELECT status FROM job_locations WHERE id = $1 AND tenant_id = $2', [job2, seedData.tenant.id]))[0].status).toBe('pending');

    // Go back online and bind this replay to its own committed receipt.
    const replayResponse = waitForStatus(job2, 'en_route');
    await page.context().setOffline(false);
    await verifyStatus(await replayResponse, job2, 'en_route');

    // Wait for the queue to sync and UI to refresh
    await expect(offlineIndicator).toBeHidden({ timeout: 10000 });

    expect(requestKeys.size).toBe(4);

    // Verify status persisted and refreshed from server
    await expect(job2Card.locator('.job-status')).toHaveText('en route', { timeout: 10000 });

  });
});
