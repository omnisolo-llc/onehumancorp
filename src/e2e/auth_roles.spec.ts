import { test, expect } from './fixtures';

test.describe('Database-seeded authentication', () => {
  test('admin user logs in through the real UI', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });

  test('regular team member logs in through the real UI', async ({ memberPage }) => {
    await expect(memberPage.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(memberPage.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  });
});
