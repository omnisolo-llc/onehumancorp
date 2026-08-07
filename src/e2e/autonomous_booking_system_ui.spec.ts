import { test, expect } from './fixtures';

test.describe('Autonomous Booking System UI', () => {
  test('Owner Admin Dashboard', async ({ page }) => {
    await page.goto(`/admin/bookings`);
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();

    const newResNameInput = page.getByPlaceholder('Resource Name');
    await newResNameInput.fill('Studio A');
    await page.getByRole('button', { name: 'Add Resource' }).click();
    await expect(page.getByText('Studio A').first()).toBeVisible();
  });
});
