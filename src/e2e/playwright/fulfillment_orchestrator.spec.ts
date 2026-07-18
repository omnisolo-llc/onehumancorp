import { test, expect } from '@playwright/test';

test.describe('Fulfillment Orchestrator', () => {
  const tenantId = 'fulfillment-e2e-tenant';

  test('should display a fulfillment draft and allow approval', async ({ page, request }) => {
    // 1. User logs in
    await page.goto('/');

    await page.evaluate((tId) => {
      localStorage.setItem('has_onboarded', 'true');
      localStorage.setItem('tenant_id', tId);
      localStorage.setItem('tenant', tId);
      localStorage.setItem('token', 'test-token');
      localStorage.setItem('user_id', 'test-user');
    }, tenantId);

    // 2. Go to dashboard
    await page.goto('/dashboard.html');

    // Wait for feed to load
    await expect(page.locator('#dashboard-title')).toBeVisible();

    // 3. Inject event by simulating via our mock api
    await request.post(`/api/dev/simulate-fulfillment-draft?tenant_id=${tenantId}`);

    // Force refresh the feed
    await page.evaluate(() => {
        if (window.loadUnifiedFeed) {
            window.loadUnifiedFeed();
        }
    });

    const card = page.getByTestId('agent-feed-card').filter({ hasText: 'Fulfillment Draft' }).first();
    await expect(card).toBeVisible({ timeout: 15000 });

    // Verify logic proofs
    await expect(card.locator('.triage-logic-proofs')).toContainText('✅ Spot reserved in calendar.');
    await expect(card.locator('.triage-logic-proofs')).toContainText('✅ Surge pricing applied (+15%).');

    // Verify it has the approve button
    const approveBtn = card.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();

    // 5. Acknowledge/Complete
    await approveBtn.click();

    // 6. Card is marked resolved (disappears)
    await expect(card).not.toBeVisible({ timeout: 15000 });
  });
});
