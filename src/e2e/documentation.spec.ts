import { test, expect } from './fixtures';

test.describe('Documentation Feature E2E', () => {
  test('should display and navigate the help center properly', async ({ page }) => {
    await page.goto('/help');

    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(page.getByPlaceholder('Search for help articles and videos...')).toBeVisible();
  });
});
