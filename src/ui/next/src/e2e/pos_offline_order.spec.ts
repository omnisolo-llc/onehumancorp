import { test, expect, devices } from '@playwright/test';

test.use({
  ...devices['iPhone 13'],
});

test.describe('POS Terminal - Offline Create Order Flow', () => {
  test('should allow creating an order offline and optimistically saving it', async ({ page }) => {
    await page.goto('http://localhost:3000/dashboard');

    const sellInPersonBtn = page.locator('#sell-in-person-btn');
    await expect(sellInPersonBtn).toBeVisible();
    await sellInPersonBtn.click();
    await expect(page).toHaveURL(/.*\/pos\/terminal/);

    // Turn off network to simulate dead zone
    await page.context().setOffline(true);

    try {
      const pinInput = page.locator('input[type="password"]');
      await pinInput.waitFor({ state: 'visible', timeout: 5000 });
      await pinInput.fill('1234');
      await page.click('button:has-text("Unlock")');
      await page.waitForTimeout(1000);
    } catch (e) {
      // Pin screen might be skipped if already authenticated
    }

    // Verify offline indicator
    await expect(page.getByText('Offline Mode Active').or(page.getByText('Offline Mode'))).toBeVisible({ timeout: 15000 });

    const isCatalogVisible = await page.locator('h3:has-text("Product Catalog")').isVisible();
    expect(isCatalogVisible).toBeTruthy();

    const productButton = page.locator('.grid.grid-cols-1.gap-3.mb-8 button').first();
    const count = await productButton.count();
    expect(count).toBeGreaterThan(0);

    await productButton.click();

    const bottomBarChargeBtn = page.locator('button', { hasText: 'Charge' }).last();
    await expect(bottomBarChargeBtn).toBeVisible();
    await bottomBarChargeBtn.click();

    // Verify Cart Drawer is open
    await expect(page.locator('h2:has-text("Current Order")')).toBeVisible();

    const cashBtn = page.locator('button', { hasText: /Record Cash Sale/ });
    await expect(cashBtn).toBeVisible();
    await cashBtn.click();

    // Optimistic UI updates
    await expect(page.getByText('Cash sale recorded offline. Will sync later.').or(page.getByText('Saved Offline - Will sync when connected'))).toBeVisible({ timeout: 15000 });

    // Reconnect to verify sync mechanism is triggered (optional here, but we check if KDS handles it ideally)
    await page.context().setOffline(false);
  });
});
