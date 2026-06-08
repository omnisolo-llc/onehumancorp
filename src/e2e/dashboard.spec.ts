import { test, expect } from './fixtures';

test.describe('Dashboard Core', () => {
  test('loads the dashboard and business snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByText('Total Sales')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();

    // Assert Growth Hub is present
    await expect(page.getByRole('heading', { name: 'Growth & Virality' })).toBeVisible();
    await expect(page.getByRole('link', { name: /Referrals/i })).toBeVisible();
    await expect(page.getByRole('link', { name: /Milestones/i })).toBeVisible();
  });

  test('navigates to login and agents screens', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();

    await page.goto('/assistant');
    await expect(page.getByRole('heading', { name: 'Jarvis Assistant' })).toBeVisible();
  });

  test('opens setup from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Launch Site' }).click();
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  });
});
