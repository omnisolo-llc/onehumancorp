import { test, expect } from '../fixtures';
import { randomUUID } from 'crypto';

test.describe('Analyst Agent Feed UI', () => {
  test('Owner sees Analyst weekly insights in the agent feed (mobile layout)', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.setViewportSize({ width: 375, height: 812 });

    const response = await page.request.post(`/api/dev/sql`, {
      data: {
        query: `
          INSERT INTO ohc_job_queue (id, tenant_id, job_type, status, next_retry_at)
          VALUES ('job-' || substr(md5(random()::text), 1, 8), 'default', 'analyst_weekly_summary', 'PENDING', NOW())
        `
      }
    });

    // Wait for the worker to process (polls every 5s)
    await page.waitForTimeout(6000);

    await page.goto('/dashboard');

    const feed = page.locator('[data-testid^="triage-card-"]').first();

    // Force strict assertion instead of flawed Promise.race
    await expect(feed).toBeVisible({ timeout: 10000 });
  });
});
