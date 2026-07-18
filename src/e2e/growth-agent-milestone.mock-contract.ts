import { test, expect } from './fixtures';
import { adminUser } from './fixtures';

test.describe('Growth Agent Milestone', () => {
  test('triggers milestone and displays "Share & Earn" card in feed', async ({ page, request, loginAs }) => {
    // 1. Log in
    await loginAs(page, adminUser);

    // 2. Trigger the milestone
    const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'e2e-tenant');
    const triggerRes = await request.post('/api/v1/growth/trigger-milestone', {
        headers: {
            'X-Tenant-ID': tenantId
        }
    });
    expect(triggerRes.ok()).toBeTruthy();

    // 3. Go to the dashboard
    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    // 4. Wait for the feed card to appear and verify contents
    const feedItem = page.locator('.triage-item', { hasText: '100 Sales Milestone reached!' }).first();
    await expect(feedItem).toBeVisible();

    // 5. Verify the "Share & Earn" button
    const shareBtn = feedItem.locator('button', { hasText: 'Share & Earn' });
    await expect(shareBtn).toBeVisible();

    // 6. Click the share button (Approve action in the background)
    await shareBtn.click();

    // 7. Verify the card gets dismissed or status appears
    await expect(feedItem).toBeHidden();
  });
});
