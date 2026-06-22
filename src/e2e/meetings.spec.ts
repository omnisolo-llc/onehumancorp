import { test, expect } from './fixtures';

test.describe('Meetings Page', () => {
  test('shows upcoming meeting and scheduler', async ({ page }) => {
    await page.goto('/calendar');
    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Morning Briefing' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Upcoming Appointments' })).toBeVisible();

    await expect(page.getByText('AI Scheduling (Zero-Setup)')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Operations Agent' })).toBeVisible();
  });

  test('opens meeting room controls', async ({ page }) => {
    await page.goto('/calendar');
    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();

    const aiSchedulingToggle = page.locator('header button').first();
    await expect(aiSchedulingToggle).toBeVisible();
    await expect(aiSchedulingToggle.locator('span')).toHaveClass(/translate-x-5/);

    await aiSchedulingToggle.click();
    await expect(aiSchedulingToggle.locator('span')).toHaveClass(/translate-x-0/);
  });
});
