import { test, expect } from '@playwright/test';

test.describe('Agentic Invoicing System E2E', () => {
  test('should verify invoice generator page renders properly and lists invoices', async ({ page }) => {
    // Navigate to the invoice generator page
    await page.goto('/invoice-generator');

    // Verify header
    await expect(page.locator('h1')).toHaveText('Invoice Generator');
    await expect(page.locator('h2', { hasText: 'Invoices' })).toBeVisible();

    // Verify loading state resolves
    await expect(page.locator('text=Loading invoices...')).toBeHidden();

    // Verify Create New Invoice button exists
    await expect(page.locator('button:has-text("Create New Invoice")')).toBeVisible();

    // Click to show form
    await page.locator('button:has-text("Create New Invoice")').click();

    // Verify form elements exist
    await expect(page.locator('input[placeholder="e.g. Acme Corp"]')).toBeVisible();
    await expect(page.locator('textarea[placeholder="e.g. Website Redesign and SEO Optimization"]')).toBeVisible();
    await expect(page.locator('input[placeholder="e.g. 1500.00"]')).toBeVisible();
    await expect(page.locator('button:has-text("Generate Shareable Invoice")')).toBeVisible();
  });
});
