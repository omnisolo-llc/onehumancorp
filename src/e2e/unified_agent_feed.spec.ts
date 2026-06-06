import { test, expect } from './fixtures';

test.describe('Unified Agent Feed', () => {
  test('should display agent feed and allow interaction', async ({ page }) => {

    // Ensure we are using the seeded e2e tenant explicitly to fetch the seed data
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });

    // Go to dashboard
    await page.goto('/dashboard');

    // Verify we are on dashboard and the Unified Agent Feed is present
    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    await expect(page.getByText(/All caught up!|Requires Review|Loading Agent Proposals/).first()).toBeVisible();
    await page.getByRole('button', { name: 'Activity Feed' }).click();
    await expect(page.getByRole('button', { name: 'Activity Feed' })).toBeVisible();
  });

  test('should display social post variants from Promoter Agent and schedule them', async ({ page, request }) => {
    // Generate a payload to test the new Promoter Agent feed card
    const orgId = 'e2e-tenant';

    // Call the catalog API directly to trigger a product created event which the MarketingAgent processes
    const res = await request.post('/api/catalog/product', {
      headers: {
        'x-tenant-id': orgId,
        'x-user-id': 'e2e-admin-user',
        'Authorization': `Bearer fake-jwt-for-e2e`,
      },
      data: {
        name: 'Agent Test Product',
        price: '19.99',
        description: 'A test product to trigger the agent.',
        item_type: 'Physical',
      }
    });

    // Wait a little for the agent to process the event and create an approval request
    await page.waitForTimeout(2000);

    // Go to dashboard
    await page.addInitScript(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('user_id', 'e2e-admin-user');
    });
    await page.goto('/dashboard');

    await expect(page.locator('button', { hasText: 'Proposals' }).first()).toBeVisible();

    // We expect the Promoter Agent to have generated variants
    await expect(page.getByText('Suggested Variants')).toBeVisible({ timeout: 10000 });

    // The Schedule button should be present for social posts
    const scheduleBtn = page.getByRole('button', { name: 'Schedule' }).first();
    await expect(scheduleBtn).toBeVisible();

    // Click Schedule
    await scheduleBtn.click();

    // Card should disappear from Proposals after approval
    await expect(scheduleBtn).not.toBeVisible();
  });
});
