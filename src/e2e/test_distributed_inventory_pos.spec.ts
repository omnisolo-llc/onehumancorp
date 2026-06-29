import { test, expect } from './fixtures';

test.describe('Distributed POS Architecture - Inventory Locking Real API', () => {

  test('Online checkout receives error message when item is sold out due to lock contention', async ({ page, request, memberPage }) => {
    // 1. Create a product with 1 available inventory via API
    await page.goto('/products/new');
    await page.getByPlaceholder('e.g., Guitar lessons for beginners, 1 hour').fill('Scarce Product');
    await page.getByRole('button', { name: 'Generate' }).click();
    await expect(page.getByRole('button', { name: 'Looks Good' })).toBeVisible({ timeout: 15000 });

    // Set inventory to exactly 1
    await page.evaluate(() => {
        const input = document.querySelector('input[name="inventory"]') as HTMLInputElement;
        if (input) {
            input.value = '1';
            input.dispatchEvent(new Event('input', { bubbles: true }));
        }
    });

    await page.getByRole('button', { name: 'Looks Good' }).click();
    await expect(page.getByText('Product Published!')).toBeVisible({ timeout: 10000 });

    // 2. Fetch the catalog to get the product_id
    const catalogRes = await request.get('/api/v1/storefront/catalog');
    let productId = '';
    if (catalogRes.ok()) {
        const catalog = await catalogRes.json();
        const product = catalog.products?.find((p: any) => p.title.includes('Scarce Product'));
        if (product) {
            productId = product.id;
        }
    }

    expect(productId).not.toBe('');

    // 3. Initiate a cart checkout to reserve the inventory via real backend API
    const cartRes = await request.post('/api/v1/cart/items', {
        data: {
            product_id: productId,
            quantity: 1
        }
    });
    expect(cartRes.ok()).toBeTruthy();

    // The cart API internally calls reserve_inventory with 900 seconds TTL.
    // So the item is now locked.

    // 4. In a separate flow, try to checkout the same item
    await memberPage.goto('/storefront');
    await memberPage.getByRole('button', { name: 'Add to Cart' }).first().click();

    await memberPage.getByRole('button', { name: 'Checkout' }).click();

    // Since the first checkout locked the inventory, the second checkout should show the contention error
    await expect(memberPage.getByText(/Lock contention on limited item|Item is currently being checked out|sold out/i)).toBeVisible({ timeout: 15000 });
  });
});
