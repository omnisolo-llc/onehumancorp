import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {
  test('Owner Admin Dashboard interaction', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/catalog.html');
    const catalogHeading = page.getByRole('heading', { name: 'Catalog' }).or(page.getByRole('heading', { name: 'Products' }));
    await expect(catalogHeading).toBeVisible();

    const addProductBtn = page.getByRole('button', { name: /Add Product/i }).or(page.getByRole('button', { name: /New/i }));
    await addProductBtn.click();

    const titleInput = page.locator('input[name="title"]');
    await expect(titleInput).toBeVisible();
    await titleInput.fill('New Tutor Leo');

    const typeSelect = page.locator('select[name="product_type"]');
    await typeSelect.selectOption('booking');
  });

  test('Public Booking Form Flow (Unauthenticated)', async ({ page }) => {
    await page.goto(`/storefront.html?tenant=e2e-tenant`);
    await expect(page.locator('body')).toBeVisible();

    const bookingItem = page.getByText(/Cake Decorating Class/i).first();
    await expect(bookingItem).toBeVisible();
    await bookingItem.click();

    const bookBtn = page.getByRole('button', { name: /Book/i }).first();
    await expect(bookBtn).toBeVisible();
    await bookBtn.click();
  });
});
