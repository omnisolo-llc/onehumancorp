import { test, expect } from './fixtures';

test.describe('Terminal Edge Sync', () => {
  test('Terminal setting renders', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto(`/settings/terminal`);
    await expect(page.getByRole('heading', { name: 'Terminal' })).toBeVisible({ timeout: 10000 });
  });
});
