import { test, expect } from '@playwright/test';

test.describe('Autonomous AI Tax and Compliance Engine', () => {
  test('Checkout flow evaluates real-time tax', async ({ page }) => {
    // Navigate to a business storefront (e2e-tenant)
    await page.goto('/store/e2e-tenant');

    // Wait for product load
    await page.waitForSelector('.product-card');

    // Add a product to cart
    await page.click('.product-card button:has-text("Add to Cart")');
    await page.click('button:has-text("Checkout")');

    // Enter shipping details (mocking UI form)
    await page.fill('input[name="country"]', 'US');
    await page.fill('input[name="state"]', 'CA');
    await page.fill('input[name="zip"]', '90210');

    // Trigger update (assume form blur or update button triggers the API)
    await page.getByText('Update Total').click().catch(() => {});

    // Assert tax line item appears
    await expect(page.locator('.tax-line-item')).toBeVisible({ timeout: 10000 });
    const taxText = await page.locator('.tax-line-item .amount').textContent();
    expect(taxText).toMatch(/\$\d+\.\d{2}/);
  });

  test('Tax Health Dashboard displays compliance alerts', async ({ page }) => {
    // Log in as business owner
    await page.goto('/login');
    await page.fill('input[type="email"]', 'owner@e2e-tenant.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');

    // Navigate to Tax Health Dashboard
    await page.goto('/dashboard/tax-health');

    // Verify widget loads
    await expect(page.locator('text=Tax Liability Summary')).toBeVisible();

    // Verify plain-language threshold warning
    const alertsSection = page.locator('.tax-alerts-section');
    await expect(alertsSection).toBeVisible();
    const alertText = await alertsSection.textContent();
    // It should say something like "You are nearing the economic nexus"
    expect(alertText).toMatch(/economic nexus/i);
  });
});
