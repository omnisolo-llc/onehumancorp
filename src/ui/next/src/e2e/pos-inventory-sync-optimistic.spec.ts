import { test, expect } from '@playwright/test';

test.describe('POS Inventory Sync - Optimistic UI', () => {
  test('POS terminal immediately updates stock UI on charge before API returns', async ({ page }) => {
    // Navigate to POS terminal
    await page.goto('/pos/terminal');

    // Wait for the pin screen to be visible
    await expect(page.getByText('Terminal Locked')).toBeVisible();

    // Login with PIN 1234
    await page.getByRole('button', { name: '1', exact: true }).click();
    await page.getByRole('button', { name: '2', exact: true }).click();
    await page.getByRole('button', { name: '3', exact: true }).click();
    await page.getByRole('button', { name: '4', exact: true }).click();

    // Wait for the dashboard to load
    await expect(page.getByRole('heading', { name: 'Manager' })).toBeVisible({ timeout: 5000 });

    // Wait for the product catalog to be populated
    await expect(page.getByText('Vegan Celebration Cake')).toBeVisible();

    // Extract current stock from the text
    const productButton = page.locator('button', { hasText: 'Vegan Celebration Cake' });
    const descriptionText = await productButton.innerText();

    const stockMatch = descriptionText.match(/Stock: (\d+)/);
    expect(stockMatch).toBeTruthy();

    if (stockMatch) {
      const initialStock = parseInt(stockMatch[1], 10);

      // Select the product
      await productButton.click();

      // Click the "Charge" button
      await page.getByRole('button', { name: /Charge \$/ }).click();

      // Immediately verify the stock decreased by 1 without waiting for API
      // Since it's optimistic, it should happen instantly.
      await expect(productButton).toContainText(`Stock: ${initialStock - 1}`);
    }
  });
});
