import { test, expect } from './fixtures';

test.describe('Dashboard Core', () => {
  test('loads the dashboard and business snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByText("Today's Sales")).toBeVisible();
    await expect(page.getByText('Business Snapshot')).toBeVisible();
  });

  test('navigates to login and agents screens', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();

    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('opens setup from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Launch Site' }).click();
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  });
});
