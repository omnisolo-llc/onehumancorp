import { test, expect } from '@playwright/test';

test.describe('CUJ: Inventory and Supply Chain Management', () => {
  test('Persona: Business Owner approves an automated purchase order', async ({ page }) => {
    // Navigate to the inventory page
    await page.goto('/inventory');

    // Check page title
    await expect(page.getByRole('heading', { name: 'Inventory' })).toBeVisible();

    // Check low stock alert is present
    await expect(page.getByRole('heading', { name: 'Low Stock Alerts' })).toBeVisible();

    // Verify the simulated low stock item is displayed
    const alertCard = page.locator('[data-testid="alert-card-mat1"]');
    await expect(alertCard).toBeVisible();
    await expect(alertCard.locator('h3')).toContainText('Cocoa Powder');

    // Approve and Pay
    const approveBtn = page.locator('[data-testid="approve-btn-mat1"]');
    await approveBtn.click();

    // Verify success message
    const successMsg = page.locator('[data-testid="success-msg"]');
    await expect(successMsg).toBeVisible();
    await expect(successMsg).toContainText('Approved Purchase Order for mat1');

    // Verify the card is removed
    await expect(alertCard).toBeHidden();

    // Verify all caught up message
    await expect(page.locator('text=All stock levels are looking good!')).toBeVisible();
  });
});
// TODO: Validate E2E tests in a proper CI environment, as local sandbox runs encounter a Docker/PGVector permission issue.
