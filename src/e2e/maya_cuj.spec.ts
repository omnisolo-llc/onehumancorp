import { expect, test } from './fixtures';

test.describe('Maya the Home Baker CUJ', () => {
  test('Step 1: Maya can configure her business name', async ({ page }) => {
    await page.goto('/settings');
    const nameInput = page.getByLabel('Business Name');
    await nameInput.fill("Maya's Custom Cakes");
    await page.getByRole('button', { name: 'Save Changes' }).click();
    await expect(page.getByText('Settings saved successfully')).toBeVisible();

    // Verify persistence after navigation
    await page.goto('/dashboard');
    await page.goto('/settings');
    await expect(page.getByLabel('Business Name')).toHaveValue("Maya's Custom Cakes");
  });

  test('Step 2: Maya can add a custom cake product', async ({ page }) => {
    await page.goto('/products');
    await page.getByRole('button', { name: 'Add Product' }).click();
    await page.getByLabel('Product Name').fill('Custom Vegan Chocolate Cake');
    await page.getByLabel('Price').fill('50.00');
    await page.getByRole('button', { name: 'Save Product' }).click();

    await expect(page.getByText('Custom Vegan Chocolate Cake')).toBeVisible();
    await expect(page.getByText('$50.00')).toBeVisible();
  });

  test('Step 3: Maya can see her products on her public storefront', async ({ page }) => {
    // Assuming 'e2e' is the tenant slug for testing
    await page.goto('/storefront/e2e');
    await expect(page.getByRole('heading', { name: "Maya's Custom Cakes" })).toBeVisible();
    await expect(page.getByText('Custom Vegan Chocolate Cake')).toBeVisible();
    await expect(page.getByText('$50.00')).toBeVisible();
  });

  test('Step 4: Customers can place an order on the storefront', async ({ page }) => {
    await page.goto('/storefront/e2e');
    await page.getByRole('button', { name: 'Order' }).first().click();
    await page.getByRole('button', { name: 'Pay with Card' }).click();
    await expect(page.getByText('Order confirmed!')).toBeVisible();
  });

  test('Step 5: Maya can see and verify the order on her dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('link', { name: 'Orders' }).click();

    // Verify the specific order exists
    const orderRow = page.locator('.order-row', { hasText: 'Custom Vegan Chocolate Cake' }).first();
    await expect(orderRow).toBeVisible();
    await expect(orderRow.getByText('PAID')).toBeVisible();
  });
});
