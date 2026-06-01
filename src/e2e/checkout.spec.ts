import { test, expect } from './fixtures';

test.describe('Checkout Flow & Mercado Pago', () => {

  test('should display Checkout heading', async ({ page }) => {
    await page.goto('/checkout');
    await expect(page.locator('h1').filter({ hasText: 'Checkout' })).toBeVisible();
  });

  test('should display "Pay with Mercado Pago" button', async ({ page }) => {
    await page.goto('/checkout');
    const mercadoPagoButton = page.locator('button:has-text("Pay with Mercado Pago")');
    await expect(mercadoPagoButton).toBeVisible();
  });

  test('should show Success Modal when clicking "Pay with Mercado Pago"', async ({ page }) => {
    await page.goto('/checkout');
    const mercadoPagoButton = page.locator('button:has-text("Pay with Mercado Pago")');

    // Mock window alert
    page.on('dialog', dialog => dialog.accept());

    await mercadoPagoButton.click();
    await expect(page.locator('h2:has-text("Payment Successful!")')).toBeVisible();
  });

  test('should copy referral link in Success Modal', async ({ page, context }) => {
    // grant clipboard permissions
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await page.goto('/checkout');
    const mercadoPagoButton = page.locator('button:has-text("Pay with Mercado Pago")');

    // Mock window alert
    page.on('dialog', dialog => dialog.accept());

    await mercadoPagoButton.click();
    await expect(page.locator('h2:has-text("Payment Successful!")')).toBeVisible();

    const copyButton = page.locator('button', { hasText: 'Copy' }).first();
    await expect(copyButton).toBeVisible();
    await copyButton.click();

    // Give it a tiny bit of time for state to update
    await page.waitForTimeout(500);
    await expect(page.locator('button', { hasText: 'Copied!' })).toBeVisible();
  });

  test('should navigate to dashboard after successful payment', async ({ page }) => {
    await page.goto('/checkout');
    const mercadoPagoButton = page.locator('button:has-text("Pay with Mercado Pago")');

    // Mock window alert
    page.on('dialog', dialog => dialog.accept());

    await mercadoPagoButton.click();
    await expect(page.locator('h2:has-text("Payment Successful!")')).toBeVisible();

    const continueButton = page.locator('button:has-text("Continue to Dashboard")');
    await continueButton.click();

    // The router.push('/dashboard') is mocked or real?
    // We expect it to try to navigate or at least not fail.
    // If it navigates to dashboard:
    await expect(page).toHaveURL(/.*dashboard/);
  });

});
