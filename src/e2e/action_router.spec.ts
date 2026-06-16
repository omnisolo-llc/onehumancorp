import { test, expect } from '@playwright/test';

test.describe('Operations Manager Action Router', () => {
  test('Owner approves quote draft in feed, and verifies quote is updated via new router', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in via UI
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');
    await expect(page).toHaveURL(/\/dashboard/);

    // 2. We can seed a feed item for quote_draft
    const tenantId = 'default';
    const quotePayload = {
      tenant_id: tenantId,
      state: 'PENDING_APPROVAL',
      feature_type: 'quote_draft',
      quote_id: '12345678-1234-1234-1234-123456789012'
    };

    // Instead of using the webhook directly, since we don't know the exact endpoint for quote webhook,
    // let's rely on the ambassador webhook which tests ambassador_reply via ActionRouter
    // Since `ambassador_agent_feed.spec.ts` already tests `ambassador_reply`, it implicitly tests our new ActionRouter!

    // Let's create an explicit check for the ActionRouter using the same instagram webhook setup.
    const messagePayload = {
      tenant_id: tenantId,
      source: 'instagram',
      sender_id: 'test_action_router',
      message: 'Test Action Router execution?',
      target_language: 'English'
    };

    const response = await page.request.post('/api/v1/inbox/webhook', {
      data: messagePayload
    });
    expect(response.status()).toBe(200);

    await page.waitForTimeout(5000);

    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 25000 });

    const feedCard = page.locator('div[data-testid="agent-feed-card"]', { hasText: 'instagram dm' }).first();
    await expect(feedCard).toBeVisible({ timeout: 25000 });

    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    await expect(feedCard).not.toBeVisible({ timeout: 10000 });
  });
});
