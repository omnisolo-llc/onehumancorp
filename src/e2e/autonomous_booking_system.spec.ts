import { test, expect } from './fixtures';

test.describe('Autonomous Booking System CUJ', () => {
  test('Owner sets up a new service and availability via UI', async ({ page }) => {
    await page.goto('/admin/bookings');
    await expect(page.getByRole('heading', { name: 'Booking Management' })).toBeVisible();

    const newResNameInput = page.getByPlaceholder('Resource Name');
    await newResNameInput.fill('Leo Tutor');
    await page.getByRole('button', { name: 'Add Resource' }).click();
    await expect(page.getByText('Leo Tutor').first()).toBeVisible();
  });
});
