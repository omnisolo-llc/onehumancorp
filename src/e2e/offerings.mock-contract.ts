import { test, expect } from './fixtures';

test.describe('Offering Creation CUJ', () => {
  test('User can create an offering via conversational prompt', async ({ page }) => {
    await page.goto('/login');
    await page.evaluate(() => {
        localStorage.setItem('tenant_id', 'e2e-tenant');
        localStorage.setItem('user_name', 'E2E User');
        localStorage.setItem('has_onboarded', 'true');
    });

    await page.goto('/dashboard');

    await expect(page.locator('h1.app-title').first()).toBeVisible({ timeout: 10000 });

    const fab = page.locator('a[aria-label="Add Offering"]');
    await expect(fab).toBeVisible();
    await fab.click();

    await expect(page.locator('h2', { hasText: 'What do you want to offer?' }).first()).toBeVisible();

    await page.fill('textarea[placeholder="e.g., Guitar lessons for beginners, 1 hour"]', 'Guitar lessons for beginners, 1 hour');

    await page.click('button:has-text("Generate")');

    const publishButton = page.locator('button', { hasText: /Publish/ }).first();
    await expect(publishButton).toBeVisible({ timeout: 15000 });

    const inputs = page.locator('input[type="text"]');
    await inputs.nth(1).fill('45.00');

    await publishButton.click();

    await expect(page.locator('text=Your new product is now live on your storefront.').first()).toBeVisible({ timeout: 10000 });
  });
});
