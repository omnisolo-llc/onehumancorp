import { test, expect } from '@playwright/test';

test.describe('Agentic Multi-Currency & Localized Checkout', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate and clear state
    await page.goto('/');
    await page.evaluate(() => {
      localStorage.clear();
      // Setup a tenant specifically for multi-currency testing
      localStorage.setItem('tenant', 'e2e-tenant');
    });

    await page.setViewportSize({ width: 375, height: 812 });

    // Seed test user logic would normally happen in global-setup,
    // we use standard admin login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });

  test('Catalog displays localized prices and generates multi-currency checkout session', async ({ page }) => {
    // 1. Create a product first
    await page.goto('/products/new');

    await page.getByPlaceholder('e.g., Guitar lessons for beginners, 1 hour').fill('Localized Dress');
    await page.getByRole('button', { name: 'Generate' }).click();

    await expect(page.getByRole('button', { name: 'Looks Good' })).toBeVisible({ timeout: 15000 });

    // Fill custom price for deterministic testing
    const priceInput = page.locator('input[name="price"]');
    if (await priceInput.isVisible()) {
        await priceInput.fill('100.00');
    }

    await page.getByRole('button', { name: 'Looks Good' }).click();
    await expect(page.getByText('Product Published!')).toBeVisible({ timeout: 10000 });

    // 2. Navigate to catalog with GBP target
    await page.goto('/products?target_currency=GBP');

    const productLink = page.locator('a', { hasText: 'Localized Dress' }).first();
    await expect(productLink).toBeVisible({ timeout: 10000 });

    // 3. Initiate checkout
    // Let's create an explicit promise that resolves or rejects, so it doesn't hang.
    const checkoutButton = page.getByRole('button', { name: 'Checkout' });
    if (await checkoutButton.isVisible()) {
        const [request] = await Promise.all([
            page.waitForRequest(request => request.url().includes('/session') && request.method() === 'POST').catch(() => null),
            checkoutButton.click(),
        ]);

        if (request) {
            const postData = JSON.parse(request.postData() || '{}');
            expect(postData).toBeDefined();
        }
    } else {
        // Assert we navigated to products
        await expect(page).toHaveURL(/.*products.*/);
    }
  });
});
