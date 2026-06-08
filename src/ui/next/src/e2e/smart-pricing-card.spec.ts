import { test, expect } from '@playwright/test';

test.describe('Smart Pricing Action Card CUJ', () => {
  test('Owner sees high demand restock/pricing suggestion and approves it', async ({ page }) => {
    // 1. Simulate the Advisor agent detecting stagnant inventory and pushing an approval request
    // Trigger simulation via the API exposed to the frontend
    await page.request.post('/api/agents/simulate-smart-pricing', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      }
    });

    // 2. Navigate dashboard to see the feed update
    await page.goto('/dashboard');

    // 3. Verify the Smart Price Suggestion card is visible
    await expect(page.getByText('Red Dress sold out in 2 days. Demand is high.')).toBeVisible();

    // 4. Verify card contents
    await expect(page.getByText('Current Price:')).toBeVisible();
    await expect(page.getByText('$40.00')).toBeVisible();

    await expect(page.getByTestId('smart-pricing-new-price').first()).toHaveText('$46.00');
    await expect(page.getByTestId('smart-pricing-sales-projection').first()).toHaveText('+$300');
    await expect(page.getByText('Restock Quantity:')).toBeVisible();
    await expect(page.getByText('50 Units')).toBeVisible();

    // 5. Tap "Approve & Run Sale"
    const approveBtn = page.getByTestId('approve-run-sale').first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 6. Optimistic UI update should remove the card from the proposals feed
    await expect(page.getByText('Red Dress sold out in 2 days. Demand is high.')).not.toBeVisible();
  });
});
