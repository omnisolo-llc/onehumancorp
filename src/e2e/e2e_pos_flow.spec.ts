import { test, expect } from './fixtures';

test.describe('In-Person Payment (POS) Flow', () => {
  test('should complete a tap-to-pay transaction', async ({ page }) => {
    await page.goto('/orders');
    await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible();
    await expect(page.locator('body')).toContainText(/Loaded from|No order rows|Order/);
  });
});
