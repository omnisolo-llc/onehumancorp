import { test, expect } from '@playwright/test';

test.describe('Agentic Inventory Alert Feed Component', () => {
  // Generate a random UUID to avoid conflicts
  const testId = `test-inventory-alert-${Math.floor(Math.random() * 1000000)}`;

  test('displays inventory alert in Agent Feed and allows approval', async ({ page }) => {
    // Navigate to a page to get the browser context
    await page.goto('/dashboard');

    // Create an inventory alert through the real backend API route we created
    const simulateResponse = await page.request.post('/api/agents/approvals/simulate-inventory-alert', {
      headers: {
        'x-tenant-id': testId
      }
    });
    expect(simulateResponse.ok()).toBeTruthy();

    // Now navigate to the dashboard using this specific tenant ID
    await page.addInitScript((tenant) => {
      window.localStorage.setItem('tenant_id', tenant);
    }, testId);

    await page.goto('/dashboard');

    // Wait for the feed to load
    await page.waitForSelector('section[aria-label="Unified Agent Feed"]');

    // Verify the inventory alert card is visible with data from the real backend
    await expect(page.getByText('Red Dress sold out in 2 days')).toBeVisible();
    await expect(page.getByText('Product ID:')).toBeVisible();

    // Find the remaining stock indicator
    await expect(page.getByText('Remaining Stock:')).toBeVisible();
    await expect(page.getByText('0', { exact: true })).toBeVisible();

    // Verify and click the Approve Restock button
    const approveBtn = page.getByTestId('approve-restock').first();
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Verify that the UI reflects the action (item disappears from pending)
    await expect(page.getByText('Red Dress sold out in 2 days')).toBeHidden();
  });
});
