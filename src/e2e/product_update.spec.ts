import { test, expect } from '@playwright/test';
import { setupAuth } from './utils';

test.describe('Product Update Flow', () => {
  setupAuth();

  test('Owner can update a product and see instant success', async ({ page }) => {
    // Navigate to products list
    await page.goto('/products');

    // Wait for the list to load and verify "Imported Products" or an existing product
    await expect(page.getByText('Imported Products')).toBeVisible();

    // Since products might be dynamically created or empty, we will create one first to ensure it's there
    await page.goto('/products/new');

    // Fill in product details
    await page.fill('input[type="text"]', 'Test E2E Product');
    await page.fill('textarea', 'A product created for E2E testing');
    const priceInputs = await page.locator('input[type="text"]').all();
    if (priceInputs.length > 1) {
        await priceInputs[1].fill('25.00'); // Assuming second text input is price
    }

    // Find category input and fill it
    const catInputs = await page.locator('input[type="text"]').all();
    if (catInputs.length > 2) {
        await catInputs[2].fill('Product');
    }

    // Publish product
    await page.getByRole('button', { name: 'Looks Good' }).click();

    // Wait for the redirect or success
    await page.waitForTimeout(4000);

    // Now go back to products list
    await page.goto('/products');

    // Find the edit button for the new product
    const editButton = page.locator('button', { hasText: 'Edit' }).first();
    await editButton.click();

    // Now on edit page
    await expect(page.getByText('Edit Product')).toBeVisible();

    // Update the product name and inventory
    const nameInput = page.locator('input[type="text"]').first();
    await nameInput.fill('Updated E2E Product');

    const inventoryInput = page.locator('input[type="number"]').first();
    await inventoryInput.fill('50');

    // Save changes
    await page.getByRole('button', { name: 'Save Changes' }).click();

    // Verify instant success message without exposing technical caching terms
    await expect(page.getByText('Product updated successfully!')).toBeVisible();
    await expect(page.getByText('Your storefront will reflect these changes momentarily.')).toBeVisible();

    // The agentic background cache invalidation and SEO pre-rendering is invisible to the user!
  });
});
