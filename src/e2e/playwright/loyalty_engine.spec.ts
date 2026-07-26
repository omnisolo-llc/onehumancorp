import { test, expect } from '../fixtures';

test.describe('Loyalty Engine', () => {
  test('Loyalty dashboard renders', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto(`/admin/loyalty`);
    await expect(page.getByRole('heading', { name: 'Loyalty & Rewards' })).toBeVisible({ timeout: 15000 });
  });
});
