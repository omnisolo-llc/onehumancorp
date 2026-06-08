import { test, expect } from '@playwright/test';

test.describe('Smart Pricing Action Card CUJ', () => {
  test('Owner sees stagnant inventory suggestion and approves it', async ({ page }) => {
    // Make sure we have a clean state by checking the price
    // But testing requires a product, wait, we don't have to verify price because we just check UI disappears.
    // Wait, the test explicitly just checks the UI. Wait, we are supposed to also check backend state changes.
    // Let's modify the test to actually fetch the product and verify the price changed.

    // First, let's create a real product to be the target of the smart pricing.
    const createResp = await page.request.post('/api/catalog', {
      data: {
        type: 'physical',
        name: 'E2E Test Scarf',
        price: 50.0,
        in_stock: true,
        inventory_count: 5
      },
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      }
    });
    // Optional: expect(createResp.ok()).toBeTruthy();

    // 1. Simulate the Advisor agent detecting stagnant inventory and pushing an approval request
    // Trigger simulation via the API exposed to the frontend
    const simResp = await page.request.post('/api/agents/simulate-smart-pricing', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      }
    });
    expect(simResp.ok()).toBeTruthy();

    // 2. Navigate dashboard to see the feed update
    await page.goto('/dashboard');

    // 3. Verify the Smart Price Suggestion card is visible
    // Depending on what product was fetched (since it fetches the first one), we use a regex or check for 'E2E Test Scarf'
    // Since simulate-smart-pricing fetches the *first* product in DB, we'll just check for 'Smart Price Suggestion:' text
    await expect(page.getByText(/Smart Price Suggestion:/)).toBeVisible();

    // 4. Verify card contents
    await expect(page.getByText('Current Price:')).toBeVisible();
    await expect(page.getByText('Suggested Price:')).toBeVisible();

    // 5. Tap "Approve & Run Sale"
    const approveBtn = page.getByTestId('approve-run-sale').first();
    await approveBtn.waitFor({ state: 'visible' });
    await approveBtn.click();

    // 6. Optimistic UI update should remove the card from the proposals feed
    await expect(page.getByTestId('approve-run-sale')).toHaveCount(0);

    // We should also verify that the backend updated the product.
    const verifyResp = await page.request.get('/api/catalog', {
      headers: {
        'x-tenant-id': 'tenant-1',
        'x-user-id': 'default'
      }
    });
    const catalogData = await verifyResp.json();
    const product = catalogData.products.find((p: any) => p.name === 'E2E Test Scarf');
    // Expect the new price to be 42.5
    expect(product.price).toBe(42.5);
  });
});
