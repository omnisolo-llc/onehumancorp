import { test, expect } from '@playwright/test';

test.describe('Dynamic Edge-Caching Storefront Architecture CUJ', () => {

  test('Persona: Business Owner creates a product and sees performance metrics', async ({ page }) => {
    // 1. Owner starts from the home page after user login via the UI
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('test@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // Now on home page
    await expect(page.getByRole('heading', { name: /Welcome/i })).toBeVisible({ timeout: 15000 });

    // 2. Owner navigates to Storefront Builder
    await page.getByRole('button', { name: /Edit Website/i }).click();

    // Ensure we landed on the builder
    await expect(page.getByRole('heading', { name: /Edit Website/i })).toBeVisible();

    // 3. Verify the "Store Performance" metric card is visible
    await expect(page.getByRole('heading', { name: /Store Performance/i })).toBeVisible();
    await expect(page.getByText('Global Edge Active')).toBeVisible();

    // Verify fast load time by clicking the speed test button
    const speedBtn = page.getByRole('button', { name: /Run Speed Test/i });
    await expect(speedBtn).toBeVisible();
    await speedBtn.click();

    // Expect the simulate check to show fast speed
    await expect(page.locator('#store-load-speed')).toHaveText('42', { timeout: 5000 });

    // 4. Owner navigates to Add Product (to trigger operations cache invalidation hook)
    await page.getByRole('button', { name: /Menu/i }).click();
    await page.getByText('Add Product').click();

    await expect(page.getByRole('heading', { name: /Add to Catalog/i })).toBeVisible();

    // Fill in product data
    await page.locator('#item-name').fill('Vegan Cupcake');
    await page.locator('#item-price').fill('4.50');
    await page.locator('#item-desc').fill('Delicious vanilla vegan cupcake.');

    const addBtn = page.getByRole('button', { name: /Save Item/i });
    await expect(addBtn).toBeVisible();
    await addBtn.click();
  });
});
