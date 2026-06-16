import { test, expect } from '@playwright/test';

test.describe('Operations Manager Action Router', () => {
  test('Owner approves an ambassador reply in feed, and verifies inbox message is updated via new router', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in via UI
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@ohc.local');
    await page.fill('input[type="password"]', 'changeme');
    await page.click('button:has-text("Log In")');
    await expect(page).toHaveURL(/\/dashboard/);

    // 2. Trigger an action that creates an agent feed item via webhook
    const tenantId = 'default';
    const messagePayload = {
      tenant_id: tenantId,
      source: 'instagram',
      sender_id: 'test_action_router_e2e',
      message: 'Can you verify the ActionRouter works?',
      target_language: 'English'
    };

    const response = await page.request.post('/api/v1/inbox/webhook', {
      data: messagePayload
    });
    expect(response.status()).toBe(200);

    // Wait for the triage agent to create the feed item
    await page.waitForTimeout(6000);

    // 3. Approve the action in the Agent Feed
    await page.goto('/feed');
    await expect(page.getByTestId('agent-feed')).toBeVisible({ timeout: 25000 });

    const feedCard = page.locator('div[data-testid="agent-feed-card"]', { hasText: 'instagram dm' }).first();
    await expect(feedCard).toBeVisible({ timeout: 25000 });

    const approveBtn = feedCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify it disappears from the feed
    await expect(feedCard).not.toBeVisible({ timeout: 10000 });

    // 4. Verify the target entity is updated
    // Go to Inbox and verify the message is marked as "replied"
    await page.goto('/inbox');
    await expect(page.getByTestId('inbox-list')).toBeVisible({ timeout: 15000 });

    // Assuming we have an inbox row for the test message
    const inboxRow = page.locator('div[data-testid="inbox-row"]', { hasText: 'test_action_router_e2e' }).first();
    await expect(inboxRow).toBeVisible({ timeout: 15000 });

    // We can click the row and verify the status badge or draft reply
    await inboxRow.click();

    // Check if the reply was actually sent / marked replied
    const statusBadge = page.getByTestId('inbox-status-badge');
    await expect(statusBadge).toContainText('Replied', { ignoreCase: true });
  });
});
