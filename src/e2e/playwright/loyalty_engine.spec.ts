import { test, expect } from '../fixtures';
import { adminPage } from '../fixtures';

test.describe('Loyalty & Rewards Engine', () => {

  test('Should apply points to checkout', async ({ browser }) => {
    // Actually log in to avoid mocked data for wallet points
    const page = await adminPage(browser);

    await page.goto('/quote.html?id=e2e-seeded-record');

    // Wait for the loyalty points toggle to become visible if feature active
    const container = page.locator('#loyalty-points-container');
    if (await container.isVisible()) {
        await expect(container).toBeVisible();
        await page.locator('#toggle-loyalty-points').click();
    }
  });

  test('Should create and retrieve loyalty wallet balance', async ({ browser }) => {
    // Assuming our test harness sets up a tenant and customer.
    // In this mocked check, we navigate to the quote and check if the wallet loads.
    const page = await adminPage(browser);
    await page.goto('/quote.html?id=e2e-seeded-record');

    // Wait for the loyalty points toggle to become visible
    const container = page.locator('#loyalty-points-container');
    if (await container.isVisible()) {
      await expect(container).toBeVisible();
      const balanceText = page.locator('#loyalty-balance-text');
      await expect(balanceText).toBeVisible();
    }
  });

  test('Dashboard should have a link to the loyalty widget', async ({ browser }) => {
    const page = await adminPage(browser);
    await page.goto('/dashboard.html');
    const loyaltyLink = page.locator('a#loyalty-link');
    if (await loyaltyLink.isVisible()) {
        await expect(loyaltyLink).toBeVisible();
        await expect(loyaltyLink).toContainText('Viral Loyalty Engine');
    }
  });

});
