import { test, expect } from './fixtures';

test.describe('Documentation Features', () => {
  test('Help Center search and navigation', async ({ page }) => {
    await page.goto('/help');
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

    const searchInput = page.getByPlaceholder('Search for help articles...');
    await searchInput.fill('payments');

    const gettingPaidArticle = page.locator('h2', { hasText: 'Getting Paid' });
    await expect(gettingPaidArticle).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Getting Started' })).not.toBeVisible();

    await gettingPaidArticle.click();

    await expect(page).toHaveURL(/\/help\/payments/);
    await expect(page.getByRole('heading', { name: 'Getting Paid', exact: true })).toBeVisible();
    await expect(page.getByText('Getting paid is the most exciting part!')).toBeVisible();

    await page.getByRole('button', { name: 'Back to Help Center' }).click();
    await expect(page).toHaveURL(/\/help$/);
  });

  test('Changelog navigation', async ({ page }) => {
    await page.goto('/changelog');
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Version 1.0 (Latest)' })).toBeVisible();
    await expect(page.getByText('Interactive AI Store Builder:')).toBeVisible();
  });
});
