import { test, expect } from '@playwright/test';

test.describe('The Ambassador - Intelligent Customer Auto-Responder', () => {
  test('Owner receives Instagram DM, views draft in feed, and approves it', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in via UI
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');
    await expect(page).toHaveURL(/\/dashboard/);

    // 2. Simulate incoming Instagram DM via omnichannel webhook
    const tenantId = 'default';
    const messagePayload = {
      tenant_id: tenantId,
      source: 'instagram',
      sender_id: 'maya_bakes',
      message: 'Do you have vegan chocolate cake available for Saturday?',
      target_language: 'English'
    };

    const response = await page.request.post('/api/v1/inbox/webhook', {
      data: messagePayload
    });
    expect(response.status()).toBe(200);

    // 3. Wait for the message triage worker to process and add to feed
    await page.waitForTimeout(5000); // Give worker some time to process

    // 4. Go to Agent Feed
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 25000 });

    // Look for the specific card based on source or content
    const feedCard = page.locator('div[data-testid="agent-feed-card"]', { hasText: 'instagram dm' }).first();
    await expect(feedCard).toBeVisible({ timeout: 25000 });

    // Ensure draft text contains "vegan" (from context) or at least basic draft
    await expect(feedCard).toContainText('vegan', { ignoreCase: true });

    // 5. Click "Send Draft"
    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 6. Verify UI updates and card disappears
    await expect(feedCard).not.toBeVisible({ timeout: 10000 });
  });
});
