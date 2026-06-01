import { test, expect } from './fixtures';

test.describe('Inventory Dashboard', () => {
  test('displays low stock alerts and allows approving PO', async ({ page }) => {
    // Navigate to a blank page to set local storage first
    await page.goto('/');

    // Set the tenant ID in local storage to match the E2E seed data
    await page.evaluate(() => {
      localStorage.setItem('tenant_id', 'e2e-tenant');
      localStorage.setItem('tenant', 'e2e-tenant');
    });

    // Navigate to the inventory page
    await page.goto('/inventory');

    // Wait for the inventory page to load
    await expect(page.locator('h1')).toHaveText('Inventory');

    // Wait for the alert card to be visible
    const alertCard = page.locator('[data-testid="alert-card-mat1"]');
    await expect(alertCard).toBeVisible({ timeout: 5000 });

    // Verify content of the alert card
    await expect(alertCard.locator('h3')).toHaveText('Cocoa Powder');
    await expect(alertCard).toContainText('50 units');
    await expect(alertCard).toContainText('$45.00');

    // Click the "Approve & Pay" button
    const approveBtn = page.locator('[data-testid="approve-btn-mat1"]');
    await approveBtn.click();

    // Verify success message
    const successMsg = page.locator('[data-testid="success-msg"]');
    await expect(successMsg).toHaveText('Approved Purchase Order for mat1', { timeout: 5000 });

    // Verify the alert card is no longer visible
    await expect(alertCard).not.toBeVisible();
  });
});
