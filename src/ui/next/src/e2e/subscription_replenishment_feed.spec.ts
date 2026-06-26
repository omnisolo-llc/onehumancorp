import { expect, test } from '@playwright/test';

test.describe('Subscription Replenishment Engine Feed E2E', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should display subscription replenishment recommendation in the feed and allow approval', async ({ page }) => {
    test.setTimeout(180000);

    // 1. Log in
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Navigate to the unified agent feed
    await page.goto('/feed');

    await page.route('**/api/agent-feed*', async (route) => {
      const json = {
        items: [
          {
            id: 'req_replenish_123',
            tenant_id: 't_1',
            lifecycle_state: 'PENDING',
            feature_type: 'subscription_replenishment',
            proposed_action: {
              action_type: 'email',
              context: 'Based on this customer\'s order history and the estimated consumption rate, they are due for a replenishment. Would you like me to generate a personalized checkout link and draft an email suggesting they refill?'
            },
            context_payload: {
              feature_type: 'subscription_replenishment',
              customer_name: 'Maya Baker'
            },
            created_at: new Date().toISOString()
          }
        ],
      };
      await route.fulfill({ json });
    });

    // Reload to apply the route interception
    await page.reload();

    // Verify the subscription replenishment card is visible
    const replenishCardText = page.getByText(/Autopilot Recommendation/i);
    await expect(replenishCardText).toBeVisible({ timeout: 15000 });

    const recommendationText = page.getByText(/due for a replenishment/i);
    await expect(recommendationText).toBeVisible();

    // Verify buttons are rendered correctly
    const approveBtn = page.getByTestId('approve-subscription-replenishment');
    await expect(approveBtn).toBeVisible();
    await expect(approveBtn).toHaveText('Generate & Send Email');

    // Setup route interception for the approval decision endpoint
    await page.route('**/api/agent-feed/req_replenish_123/state', async (route) => {
      await route.fulfill({ status: 200, json: { success: true } });
    });

    // Click the approve button
    await approveBtn.click();
  });
});
