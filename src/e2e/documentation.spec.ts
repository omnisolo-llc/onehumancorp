import { test, expect } from './fixtures';

test.describe('Documentation full suite', () => {
  test('Help portal loads properly and search works', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/help');

    // Title should be present
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('Help Center');

    // Make sure search bar exists
    const searchInput = page.getByPlaceholder('Search for help articles and videos...');
    await expect(searchInput).toBeVisible();
    await searchInput.fill('Products');

    // Wait for the articles to filter
    await expect(page.getByText('Adding Products')).toBeVisible({ timeout: 10000 });
  });

  test('Changelog pulls data dynamically', async ({ page, loginAs, unlimitedAdminUser }) => {
    await loginAs(page, unlimitedAdminUser);
    await page.goto('/changelog');

    // Title should be present
    const title = page.locator('h1');
    await expect(title).toBeVisible();
    await expect(title).toContainText('Release Notes & Changelog');
  });
});
