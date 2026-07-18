import { test, expect } from './fixtures';

test.describe('The Ambassador - Intelligent Customer Auto-Responder', () => {
  test('Owner receives Instagram DM, views draft in feed, and approves it', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    test.setTimeout(180000);

    // 1. Log in via UI






    // 2. Simulate incoming Instagram DM via omnichannel webhook
    const tenantId = 'default';
    const messagePayload = {
      tenant_id: tenantId,
      channel: 'instagram',
      sender_id: 'maya_bakes',
      message: 'Do you have vegan chocolate cake available for Saturday?'
    };

    const response = await page.request.post("/api/v1/omnichannel/webhook", {
      data: messagePayload
    });
    expect(response.status()).toBe(200);

    // 3. Wait for the message triage worker to process and add to feed
    await page.waitForTimeout(5000); // Give worker some time to process

    // 4. Go to Agent Feed
    await page.goto("/feed");
    await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 25000 });

    // Look for the specific card based on source or content
    const feedCard = page.locator('div[data-testid="agent-feed-card"]', { hasText: 'New Message from' }).first();
    await expect(feedCard).toBeVisible({ timeout: 25000 });

    // Ensure draft text contains "vegan" (from context) or at least basic draft
    await expect(feedCard).toContainText('vegan', { ignoreCase: true, timeout: 25000 });

    // 5. Click "Send Draft"
    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 6. Verify UI updates and card disappears
    await expect(feedCard).not.toBeVisible({ timeout: 10000 });
  });
});
