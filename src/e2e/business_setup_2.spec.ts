import { test, expect } from './fixtures';

test.describe('Business Setup Wizard - Part 2', () => {
  test('supports the instant build entry point', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.getByRole('heading', { name: 'Describe your business in a sentence' })).toBeVisible();
    await page.getByPlaceholder(/I run a local bakery/).fill('I run a local bakery called Maya Cakes.');
    await page.getByRole('button', { name: /Launch your business in 10 minutes/ }).click();
    await expect(page.getByRole('heading', { name: 'Designing your storefront...' })).toBeVisible();
  });
});
