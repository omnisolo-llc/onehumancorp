import { expect, test } from '@playwright/test';

test.describe('Dynamic Pricing Approval', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should present a dynamic pricing recommendation and allow approval', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Seed a dynamic pricing recommendation
    await page.request.post('/api/agent-feed', {
      data: {
        event_source: "Pricing Agent",
        context_payload: {},
        proposed_action: {
          feature_type: "dynamic_pricing",
          target_id: "test-product",
          recommendation: "'Test Product' has high stock (100) but low sales. Suggest a 15% discount to clear inventory.",
          action: "create_rule",
          rule_config: {
            name: "Clearance: Test Product",
            type: "InventoryThreshold",
            config: {
              threshold: 100,
              adjustment_percent: -15.0
            }
          }
        },
        lifecycle_state: "PENDING_APPROVAL"
      }
    });

    await page.reload();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const card = page.getByTestId('dynamic-pricing-card').first();
    await expect(card).toBeVisible({ timeout: 15000 });

    await expect(card.locator('text=Pricing Strategy')).toBeVisible();
    await expect(card.locator('text=Clearance: Test Product')).toBeVisible();

    const approveBtn = card.locator('xpath=../..').getByTestId('feed-approve-btn').first();
    await expect(approveBtn).toBeVisible();
    await expect(approveBtn).toHaveText('Approve & Run Sale');

    await approveBtn.click();
    await expect(card).not.toBeVisible({ timeout: 5000 });
  });
});
