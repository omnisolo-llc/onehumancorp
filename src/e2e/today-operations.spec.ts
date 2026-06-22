import { test, expect } from '@playwright/test';

test.describe('Today Operations Dashboard CUJ', () => {
  test('Owner views the Today operations dashboard and interacts with appointments', async ({ page }) => {
    // Navigate to login
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('carlos@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Verify successful login
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Navigate to Today dashboard
    await page.goto('/today');

    // Verify header
    await expect(page.getByRole('heading', { name: "Today's Operations" })).toBeVisible();

    // Verify Morning Briefing loaded
    await expect(page.getByText('Morning Briefing')).toBeVisible();

    // Ensure data is loaded or empty state is shown
    await expect(page.getByText('Your schedule is clear for today.').or(page.locator('[data-testid^="appointment-card-"]').first())).toBeVisible();

    // We perform the click interaction if an appointment is visible. Otherwise we just pass.
    if (await page.locator('[data-testid^="appointment-card-"]').first().isVisible()) {
        const firstAppointment = page.locator('[data-testid^="appointment-card-"]').first();
        await firstAppointment.click();

        await expect(page.getByTestId('appointment-ai-summary')).toBeVisible();
        await expect(page.getByRole('button', { name: 'Message Client' })).toBeVisible();
        await expect(page.getByRole('button', { name: 'Reschedule' })).toBeVisible();

        await expect(page.getByRole('button', { name: 'Request Payment' }).or(page.getByRole('button', { name: 'View Receipt' }))).toBeVisible();

        await page.locator('button').filter({ hasText: /^$/ }).first().click();
    }
  });
});
