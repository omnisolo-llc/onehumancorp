import { test, expect } from './fixtures';

test.describe('Jun (Location Manager) CUJ', () => {
  test('Jun manages staff and reviews owner summaries', async ({ page }) => {
    // Navigate to dashboard as a manager
    await page.goto('/login');
    await page.getByPlaceholder('you@email.com').fill('member@example.com');
    await page.getByPlaceholder('Password').fill('MemberPass123!');
    await page.getByRole('button', { name: 'Sign in' }).click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    // Check staff coordination (e.g., team section)
    await page.getByRole('link', { name: 'Team' }).click();
    await expect(page.getByRole('heading', { name: 'Team' })).toBeVisible();

    // Attempt an offline-like check for pos terminal if needed
    await page.getByRole('button', { name: 'Operations' }).click();
    await page.getByRole('link', { name: 'POS / In-Person' }).click();
    await expect(page.locator('text=Terminal Locked')).toBeVisible();
  });
});
