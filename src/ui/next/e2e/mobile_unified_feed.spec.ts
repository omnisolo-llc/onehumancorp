import { test, expect } from '@playwright/test';
import { randomUUID } from 'crypto';

test.describe('Mobile Unified Agent Feed - Optimistic Sync', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should optimistically update and show pending sync when offline', async ({ page, context }) => {
    // We will just do a standard login via API to get real token
    const testTenant = 'test-e2e-tenant-' + randomUUID();
    const testUser = 'user-' + randomUUID();

    // await page.goto('/login');
    // For this app, often tests use some bypass mechanism or login form.
    // If there is no login form easily accessible, we will mock localStorage with real-looking data.
    // However, the rule says "No mocking of network requests". It does not say we can't create real data via API before the test.
    // Let's seed via the documented backend seed route:

    // Seed real data through the app instead of mocked API routes
    await page.request.post('/api/dev/seed', {
      data: {
        scenario: 'launch-readiness'
      }
    });

    await page.goto('/dashboard');

    // Make sure we are loaded
    await page.waitForSelector('text=Proposals');

    // Check if there is an approve button (from the seeded data)
    // If not, we might need to trigger an event that creates an agent feed item.

    // Since the assignment asked to use real paths, and the setup for full end-to-end feed is complex,
    // let's do an API call to the backend to create an action card for this tenant explicitly.
    const res = await page.request.post('/api/agent-feed', {
        headers: {
            'x-tenant-id': 'default',
            'x-user-id': 'test'
        },
        data: {
            event_source: 'Playwright Test',
            proposed_action: { message: 'Playwright test message', feature_type: 'general' }
        }
    });

    await page.reload();

    const approveBtn = page.getByTestId('approve-proposal').first();
    await expect(approveBtn).toBeVisible();

    // Go offline
    await context.setOffline(true);

    // Approve the action
    await approveBtn.click();

    // It should optimistically dismiss
    await expect(approveBtn).not.toBeVisible();

    // Verify "Pending Sync" pill appears
    await expect(page.locator('text=Pending Sync (1)')).toBeVisible();

    // Come back online
    await context.setOffline(false);

    // Trigger online event to start sync
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Verify sync completed and pill is gone
    await expect(page.locator('text=Pending Sync')).not.toBeVisible();
  });
});
