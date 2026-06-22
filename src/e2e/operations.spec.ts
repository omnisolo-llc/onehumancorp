import { test, expect } from '@playwright/test';

test.describe('Operations Copilot (Today View)', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate directly to the operations page for testing.
    await page.goto('/operations');
  });

  test('displays the Today header and Morning Briefing', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Today' })).toBeVisible();
    await expect(page.getByText('Your daily schedule and operations overview.')).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Morning Briefing' })).toBeVisible();
    await expect(page.getByText('You have 4 appointments today. 1 client still needs to pay their deposit.')).toBeVisible();
  });

  test('displays past, current, and future appointments with correct visual states', async ({ page }) => {
    // Past Appointment
    const pastAppt = page.locator('.opacity-60');
    await expect(pastAppt).toBeVisible();
    await expect(pastAppt.getByText('9:00 AM')).toBeVisible();
    await expect(pastAppt.getByText('Piano Lesson')).toBeVisible();
    await expect(pastAppt.getByText('Alice Smith')).toBeVisible();
    await expect(pastAppt.getByText('Paid', { exact: true })).toBeVisible();

    // Current Appointment
    const currentAppt = page.locator('.border-blue-200'); // Check for the border styling indicative of current
    await expect(currentAppt).toBeVisible();
    await expect(currentAppt.getByText('11:00 AM')).toBeVisible();
    await expect(currentAppt.getByText('Now')).toBeVisible();
    await expect(currentAppt.getByText('Guitar Lesson')).toBeVisible();
    await expect(currentAppt.getByText('Sarah Johnson')).toBeVisible();

    // Check AI Summary in Current Appointment
    await expect(currentAppt.getByText('AI Summary:')).toBeVisible();
    await expect(currentAppt.getByText('3rd lesson. Focus: Jazz scales. She struggled with chords last week.')).toBeVisible();

    // Check Action Button and Status in Current Appointment
    await expect(currentAppt.getByText('Deposit Required')).toBeVisible();
    const messageButton = currentAppt.getByRole('button', { name: 'Message Client' });
    await expect(messageButton).toBeVisible();
    await expect(messageButton).toBeEnabled();

    // Future Appointment
    const futureAppt = page.locator('.hover\\:shadow-md').filter({ hasText: '2:00 PM' });
    await expect(futureAppt).toBeVisible();
    await expect(futureAppt.getByText('Vocal Coaching')).toBeVisible();
    await expect(futureAppt.getByText('Mike Brown')).toBeVisible();
    await expect(futureAppt.getByText('Paid', { exact: true })).toBeVisible();
  });
});
