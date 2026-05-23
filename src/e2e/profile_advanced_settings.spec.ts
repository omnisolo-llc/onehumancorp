import { test, expect } from './fixtures';

test.describe('Profile Advanced Settings Toggle', () => {
  test('should open profile menu when AC button is clicked', async ({ page }) => {
    await page.goto('/dashboard');

    // Menu should be hidden initially
    const advancedSetting = page.getByRole('button', { name: 'Advanced Developer Settings' });
    await expect(advancedSetting).toBeHidden();

    // Click AC button
    await page.getByRole('button', { name: 'Profile menu' }).click();

    // Menu should now be visible
    await expect(advancedSetting).toBeVisible();
  });

  test('should toggle advanced settings on and off', async ({ page }) => {
    await page.goto('/dashboard');

    // Open menu
    await page.getByRole('button', { name: 'Profile menu' }).click();

    const toggleButton = page.getByRole('button', { name: 'Advanced Developer Settings' });

    // Initially should not be pressed
    await expect(toggleButton).toHaveAttribute('aria-pressed', 'false');

    // Click toggle
    await toggleButton.click();

    // Wait and verify it is now pressed
    await expect(toggleButton).toHaveAttribute('aria-pressed', 'true');

    // Click toggle again
    await toggleButton.click();

    // Wait and verify it is now off
    await expect(toggleButton).toHaveAttribute('aria-pressed', 'false');
  });

  test('should persist advanced settings across reloads', async ({ page }) => {
    await page.goto('/dashboard');

    // Open menu and toggle ON
    await page.getByRole('button', { name: 'Profile menu' }).click();
    await page.getByRole('button', { name: 'Advanced Developer Settings' }).click();

    // Reload page
    await page.reload();

    // Open menu again
    await page.getByRole('button', { name: 'Profile menu' }).click();

    // Check it is still pressed
    const toggleButton = page.getByRole('button', { name: 'Advanced Developer Settings' });
    await expect(toggleButton).toHaveAttribute('aria-pressed', 'true');
  });

  test('should close menu when clicked again', async ({ page }) => {
    await page.goto('/dashboard');

    const profileButton = page.getByRole('button', { name: 'Profile menu' });
    const advancedSetting = page.getByRole('button', { name: 'Advanced Developer Settings' });

    // Open menu
    await profileButton.click();
    await expect(advancedSetting).toBeVisible();

    // Close menu by clicking again
    await profileButton.click();
    await expect(advancedSetting).toBeHidden();
  });

  test.use({ viewport: { width: 375, height: 812 } });
  test('should open and be usable on mobile viewport', async ({ page }) => {
    await page.goto('/dashboard');

    const profileButton = page.getByRole('button', { name: 'Profile menu' });
    const toggleButton = page.getByRole('button', { name: 'Advanced Developer Settings' });

    // Open menu on mobile
    await profileButton.click();
    await expect(toggleButton).toBeVisible();

    // Toggle on mobile
    await toggleButton.click();
    await expect(toggleButton).toHaveAttribute('aria-pressed', 'true');
  });
});
