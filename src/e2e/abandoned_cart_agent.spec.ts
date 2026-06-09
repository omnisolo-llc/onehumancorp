import { test, expect } from './fixtures';

test.describe('Automated Cart Recovery Agent', () => {
  test('Agent automatically dispatches AI generated message for abandoned cart', async ({ page }) => {
    // 1. Navigate to the Cart Recovery page
    await page.goto('/cart-recovery');

    // Wait for the page to load and check the title
    await expect(page.getByRole('heading', { name: /Abandoned Cart Recovery/i })).toBeVisible({ timeout: 15000 });

    // Ensure the truthful count is displayed for abandoned carts (we seeded 1)
    await expect(page.getByRole('button', { name: /Send to 1 Abandoned Carts/i })).toBeVisible({ timeout: 15000 });

    // Enter test values to generate context
    await page.getByLabel(/Customer Name/i).fill('Alice');
    await page.getByLabel(/Cart Value/i).fill('$45.00');

    // 2. Click Generate AI Campaign
    await page.getByRole('button', { name: /Generate AI Campaign/i }).click();

    // Verify draft is generated (the backend mocked text or real text)
    await expect(page.locator('text=Hi Alice')).toBeVisible({ timeout: 15000 });

    // 3. Click Send to 1 Abandoned Carts
    await page.getByRole('button', { name: /Send to 1 Abandoned Carts/i }).click();

    // Verify success message
    await expect(page.locator('text=Campaign sent to 1 abandoned carts!')).toBeVisible({ timeout: 15000 });

    // Wait for the action to log
    await page.waitForTimeout(500);

    // 4. Navigate to agent-feed where the merchant can see the action log
    await page.goto('/agent-feed');

    // The feed should mention the cart recovery agent took action, verifying the whole cycle
    await expect(page.locator('body')).toContainText(/cart_recovery_sent/i, { timeout: 15000 });
  });
});
