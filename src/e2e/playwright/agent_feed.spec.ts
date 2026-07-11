import { test, expect } from '@playwright/test';

test.describe('Agent Feed', () => {
  const tenantId = 'agent-feed-test-tenant';

  test('should receive event, show in feed, and resolve card', async ({ page, request }) => {
    // 1. User logs in
    await page.goto('/');

    await page.evaluate((tId) => {
      localStorage.setItem('has_onboarded', 'true');
      localStorage.setItem('tenant_id', tId);
      localStorage.setItem('tenant', tId);
      localStorage.setItem('token', 'test-token');
      localStorage.setItem('user_id', 'test-user');
    }, tenantId);

    // 2. Go to feed
    await page.goto('/feed');

    // Wait for feed to load
    await expect(page.getByTestId('agent-feed')).toBeVisible();

    // The feed might be empty initially
    // 3. Inject event by clicking simulate button
    await page.getByTestId('simulate-ambassador-btn').click();

    // 4. Verify new card appears in UI
    const card = page.getByTestId('agent-feed-card').first();
    await expect(card).toBeVisible({ timeout: 15000 });

    // Verify it has the approve button
    const approveBtn = card.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();

    // 5. Acknowledge/Complete
    await approveBtn.click();

    // 6. Card is marked resolved (disappears)
    await expect(card).not.toBeVisible({ timeout: 15000 });

    // Verify cache invalidation (card stays gone on reload)
    await page.reload();
    await expect(page.getByTestId('agent-feed')).toBeVisible();
    await expect(card).not.toBeVisible({ timeout: 5000 });
  });
});
