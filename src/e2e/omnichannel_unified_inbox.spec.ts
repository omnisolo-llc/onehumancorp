import { test, expect } from './fixtures';

test.describe('Omnichannel Unified Inbox Event and UI', () => {
  test('displays correctly in UI', async ({ page }) => {
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
  });
});
