import { test, expect } from './fixtures';

test.describe('Pricing & Payment Routing', () => {
  test('should display pricing plans and payment security guarantee', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=100% money back guarantee. Secure SSL payments powered by Stripe.')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' }).first()).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' }).first()).toBeVisible();
  });
});
