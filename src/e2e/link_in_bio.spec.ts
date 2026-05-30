import { test, expect } from './fixtures';

test.describe('Link in Bio', () => {
  test('should display link in bio generator', async ({ page }) => {
    await page.goto('/');

    // Wait for the Dashboard to load fully
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

    await page.getByText('Link in Bio Generator').click();

    // Verify page heading
    await expect(page.getByRole('heading', { name: 'Link in Bio Generator' })).toBeVisible();
    await expect(page.getByText('Profile Settings')).toBeVisible();

    // Verify default preview state
    await expect(page.locator('h1').filter({ hasText: 'My Profile' })).toBeVisible();

    // Edit profile name
    await page.getByLabel('Profile Name').fill('Leo Guitar Tutor');
    await expect(page.locator('h1').filter({ hasText: 'Leo Guitar Tutor' })).toBeVisible();

    // Edit bio
    await page.getByLabel('Bio').fill('Learn guitar with Leo');
    await expect(page.getByText('Learn guitar with Leo')).toBeVisible();

    page.on('dialog', dialog => dialog.accept());

    // Copy link
    await page.getByRole('button', { name: 'Copy Link-in-Bio URL' }).click();
  });
});
