import { randomUUID } from 'node:crypto';
import { test, expect } from './fixtures';

test.describe('Offline-to-online QR catalog bridge', () => {
  test('merchant can generate a contextual QR code for a persisted product', async ({ page }) => {
    const name = `QR fixture ${randomUUID()}`;
    await page.goto('/products');
    await expect(page.getByRole('heading', { name: 'Products', exact: true })).toBeVisible();
    await expect(page.getByText('Catalog Products', { exact: true })).toBeVisible();

    // Create this test's own real catalog row instead of relying on an invented
    // Chocolate Cake or whichever shared product happens to sort first.
    await page.getByRole('button', { name: 'New Product', exact: true }).click();
    await page.getByLabel('Product Name', { exact: true }).fill(name);
    await page.getByLabel('Description', { exact: true }).fill('Isolated QR regression product');
    await page.getByLabel('Price', { exact: true }).fill('23.45');
    const saved = page.waitForResponse(response => response.request().method() === 'POST'
      && new URL(response.url()).pathname === '/api/v1/catalog/product');
    await page.getByRole('button', { name: 'Save', exact: true }).click();
    const response = await saved;
    expect(response.ok()).toBe(true);
    const result = await response.json();
    expect(result).toMatchObject({ success: true, product_id: expect.any(String) });
    expect(result.product_id).not.toBe('');
    await expect(page.getByText('Product created', { exact: true })).toBeVisible();

    const row = page.locator('.app-list-item').filter({ has: page.getByText(name, { exact: true }) });
    await expect(row).toHaveCount(1);
    await expect(row).toContainText('$23.45');
    await row.getByRole('button', { name: 'Generate QR Code', exact: true }).click();
    const title = page.getByRole('heading', { name: 'Checkout QR Code', exact: true });
    await expect(title).toBeVisible();
    const modal = page.locator('.fixed.inset-0').filter({ has: title });
    await expect(modal).toContainText(name);
    const image = modal.getByAltText(`QR Code for ${name}`, { exact: true });
    await expect(image).toBeVisible();
    const source = new URL((await image.getAttribute('src'))!);
    expect(source.hostname).toBe('api.qrserver.com');
    const checkout = new URL(source.searchParams.get('data')!);
    expect(checkout.origin).toBe('https://cloud.omnisolo.co');
    expect(checkout.pathname).toBe('/checkout');
    expect(checkout.searchParams.get('product_id')).toBe(result.product_id);
    await expect(modal.getByRole('button', { name: 'Save / Print', exact: true })).toBeVisible();
    await expect(modal.getByText('⚡ Powered by OmniSolo', { exact: true })).toBeVisible();
    await modal.locator('button').first().click();
    await expect(title).toBeHidden();
    // This verifies the saved product's QR destination, not provider checkout or payment.
  });
});
