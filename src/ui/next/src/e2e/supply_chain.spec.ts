import { test, expect } from '../../../../e2e/fixtures';

test.describe('Autonomous Supply Chain', () => {
  test('shows and adjusts database-backed product inventory', async ({ page }) => {
    await page.goto('/inventory');

    await expect(page.getByRole('heading', { name: 'Inventory' })).toBeVisible();
    await expect(page.getByText('Products & Variants')).toBeVisible();

    const row = page.getByTestId('product-row-e2e-product-cake');
    await expect(row.getByRole('heading', { name: 'Vegan Celebration Cake' })).toBeVisible();

    const stock = page.getByTestId('stock-count-e2e-product-cake');
    const initialStock = Number(await stock.textContent());
    expect(Number.isFinite(initialStock)).toBe(true);

    const increaseResponse = page.waitForResponse(
      response => response.url().includes('/api/v1/ui/inventory')
        && response.request().method() === 'POST',
    );
    await page.getByTestId('increase-btn-e2e-product-cake').click();
    expect((await increaseResponse).ok()).toBe(true);
    await expect(stock).toHaveText(String(initialStock + 1));

    const restoreResponse = page.waitForResponse(
      response => response.url().includes('/api/v1/ui/inventory')
        && response.request().method() === 'POST',
    );
    await page.getByTestId('decrease-btn-e2e-product-cake').click();
    expect((await restoreResponse).ok()).toBe(true);
    await expect(stock).toHaveText(String(initialStock));
  });
});
