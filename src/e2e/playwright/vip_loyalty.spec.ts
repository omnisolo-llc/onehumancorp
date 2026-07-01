import { test, expect } from '@playwright/test';
import { currentAppSmoke } from '../utils';

test.describe('Autonomous VIP Loyalty & Re-engagement System', () => {
  test('Agent Feed Card creation and approval flow', async ({ page, request }) => {
    // 1. Simulate the background job processing and placing an item in the feed
    // Because Playwright E2E tests are isolated, we seed a task via API (similar to what the background job does internally)

    // Create an agent feed item matching the exact expected shape
    const loyaltyPayload = {
      event_source: "VIP Loyalty Agent",
      context_payload: {
        feature_type: "vip_loyalty",
        customer_id: "c-123",
        customer_name: "Sarah VIP",
        message: "Want to send them a 'We Miss You' discount?"
      },
      proposed_action: {
         draft: "Hey Sarah VIP, we miss you! It's been a while since your last visit. Enjoy 15% off your next purchase with us!"
      },
      lifecycle_state: "PENDING"
    };

    const res = await request.post('/api/agent-feed', {
      data: loyaltyPayload
    });

    expect(res.ok()).toBeTruthy();

    // 2. Navigate to Dashboard (Agent Feed)
    await page.goto('/dashboard.html');

    // 3. Verify the Action Card is present
    const loyaltyCard = page.locator('div[data-testid^="triage-card-"]', { hasText: 'VIP Loyalty' }).first();
    await expect(loyaltyCard).toBeVisible({ timeout: 10000 });

    await expect(loyaltyCard.locator('h2')).toContainText('Loyalty Alert:');

    // 4. Verify text area has the draft content
    const textarea = loyaltyCard.locator('textarea[id^="vip-offer-msg-"]');
    await expect(textarea).toBeVisible();
    await expect(textarea).toHaveValue(/Hey Sarah VIP, we miss you!/);

    // 5. Simulate editing the message
    await textarea.fill("Hey Sarah VIP, 20% off just for you today!");

    // 6. Click Approve
    const approveBtn = loyaltyCard.locator('button[data-testid="feed-approve-vip-btn"]');
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // 7. Verify the item is removed from the pending feed upon approval
    // The feed reloads automatically within 500ms so we wait for the card to disappear
    await expect(loyaltyCard).not.toBeVisible({ timeout: 10000 });
  });
});
