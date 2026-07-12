import { test, expect } from '../../../../e2e/fixtures';

test.describe('Agentic Invoicing System E2E', () => {
  test('should verify invoice generator page renders properly', async ({ page }) => {
    // Navigate to the invoice generator page
    await page.goto('/invoice-generator');

    // Verify header
    await expect(page.locator('h1').first()).toHaveText('Invoice Generator');

    // Verify form elements exist
    await expect(page.locator('input[placeholder="e.g. Acme Corp"]')).toBeVisible();
    await expect(page.locator('textarea[placeholder="e.g. Website Redesign and SEO Optimization"]')).toBeVisible();
    await expect(page.locator('input[placeholder="e.g. 1500.00"]')).toBeVisible();
    await expect(page.locator('button:has-text("Generate Shareable Invoice")')).toBeVisible();
  });
});
