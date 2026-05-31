import { test, expect } from './fixtures';

test.describe('User and Team Surfaces', () => {
  test('shows team summary on dashboard', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByText('Team Members')).toBeVisible();
    await expect(page.getByText('Ongoing Tasks')).toBeVisible();
    await expect(page.getByText('Needs Your Approval')).toBeVisible();
  });

  test('shows editable profile fields in settings', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.getByRole('heading', { name: 'Profile' })).toBeVisible();
    await page.getByPlaceholder('Display Name').fill('Team User');
    await page.getByPlaceholder('Email or Username').fill('team@example.com');
    await expect(page.getByPlaceholder('Display Name')).toHaveValue('Team User');
  });

  test('shows security fields for account management', async ({ page }) => {
    await page.goto('/settings');
    await expect(page.getByRole('heading', { name: 'Security' })).toBeVisible();
    await expect(page.getByPlaceholder('Current Password')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Change' })).toBeVisible();
  });
});
