import { test, expect } from './fixtures';

test.describe('Subscriptions', () => {
  test('Subscriptions list renders', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto(`/admin/subscriptions`);
    await expect(page.getByRole('heading', { name: 'Subscriptions' })).toBeVisible({ timeout: 10000 });
  });
});
