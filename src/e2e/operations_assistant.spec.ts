import { test, expect } from './fixtures';

test.describe('Operations Assistant', () => {
  test('Leo the Tutor checks his daily schedule and operations assistant', async ({ page }) => {
    await page.goto('/calendar');
    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();

    // Verify AI Briefing Card
    await expect(page.getByText('Morning Briefing')).toBeVisible();

    // Verify upcoming appointments
    const appointmentList = page.locator('.space-y-4').first();
    await expect(appointmentList.getByText('Guitar Lesson')).toBeVisible();
    await expect(appointmentList.getByText('Plumbing Repair')).toBeVisible();

    // Click on an appointment to open the detail view
    await appointmentList.getByText('Guitar Lesson').click();

    // Verify unified card properties
    await expect(page.getByText('Sarah Connor')).toBeVisible();
    await expect(page.getByText('3rd lesson. Focus: Jazz scales.')).toBeVisible();
    await expect(page.getByText('unpaid')).toBeVisible();

    // Verify action buttons
    await expect(page.getByRole('button', { name: 'Message Client' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Reschedule' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Request Payment' })).toBeVisible();
  });
});
