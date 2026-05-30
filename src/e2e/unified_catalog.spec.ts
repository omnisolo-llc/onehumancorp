import { test, expect } from './fixtures';

test.describe('Unified Product & Service Catalog', () => {
  test('adds a service offering successfully via mobile UI', async ({ page }) => {
    await page.goto('/website-builder');

    // Simulate showing the dashboard to make bottom nav visible for adding product
    await page.evaluate(() => {
      // @ts-ignore
      showScreen('dashboard-screen');
    });

    const addProductBtn = page.locator('text="Add Product"').filter({ visible: true }).first();
    await addProductBtn.waitFor({ state: 'visible', timeout: 30000 });
    await addProductBtn.click();

    // Verify the add-item-screen is visible
    const addItemScreen = page.locator('#add-item-screen');
    await expect(addItemScreen).toBeVisible();
    await expect(addItemScreen.getByRole('heading', { name: 'Add to Catalog' })).toBeVisible();

    // Fill the form as a Service
    await page.locator('input[type="radio"][value="service"]').click();

    // Check if dynamic field appears
    const durationInput = page.locator('#item-duration');
    await expect(durationInput).toBeVisible();
    await durationInput.fill('60');

    await page.locator('#item-name').fill('Guitar Lesson');
    await page.locator('#item-price').fill('50.00');
    await page.locator('#item-desc').fill('One hour of acoustic guitar lessons.');

    // Save and verify successful save (alert is shown)
    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Saved Guitar Lesson successfully!');
      await dialog.accept();
    });

    await page.locator('button:has-text("Save Item")').click();

    // Ensure we navigated back to dashboard
    await expect(page.locator('#dashboard-screen')).toBeVisible();
  });

  test('adds a physical product successfully via mobile UI', async ({ page }) => {
    await page.goto('/website-builder');

    // Simulate showing the dashboard
    await page.evaluate(() => {
      // @ts-ignore
      showScreen('dashboard-screen');
    });

    const addProductBtn = page.locator('text="Add Product"').filter({ visible: true }).first();
    await addProductBtn.waitFor({ state: 'visible', timeout: 30000 });
    await addProductBtn.click();

    // Verify the add-item-screen is visible
    const addItemScreen = page.locator('#add-item-screen');
    await expect(addItemScreen).toBeVisible();

    // The default is product, so duration should be hidden
    const durationInput = page.locator('#item-duration');
    await expect(durationInput).not.toBeVisible();

    await page.locator('#item-name').fill('Handmade Soap');
    await page.locator('#item-price').fill('15.00');

    page.on('dialog', async dialog => {
      expect(dialog.message()).toContain('Saved Handmade Soap successfully!');
      await dialog.accept();
    });

    await page.locator('button:has-text("Save Item")').click();

    // Ensure we navigated back to dashboard
    await expect(page.locator('#dashboard-screen')).toBeVisible();
  });
});
