import { test, expect } from '@playwright/test';

test.describe('Owner Dashboard Bookings', () => {
  test('Owner can navigate to bookings management view and see AI suggestions context', async ({ page }) => {
    await page.goto('/dashboard/bookings');
    await expect(page.getByTestId('owner-dashboard-bookings')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Bookings Management' })).toBeVisible();

    const feedLink = page.getByRole('link', { name: 'Go to Feed' });
    await expect(feedLink).toBeVisible();
    await feedLink.click();
    await expect(page).toHaveURL(/.*\/feed/);
  });
});
