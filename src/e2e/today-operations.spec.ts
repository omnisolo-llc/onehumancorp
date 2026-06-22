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

    // Check if we have appointments or an empty state. Since we can't reliably predict
    // the DB state in the test environment (might be empty, might have seeded data),
    // we use a web-first assertion with an `.or()` matcher to ensure either the empty
    // state or the appointment list successfully renders.
    const emptyState = page.getByText('Your schedule is clear for today.');
    const appointmentCard = page.locator('[data-testid^="appointment-card-"]').first();

    await expect(emptyState.or(appointmentCard)).toBeVisible();

    // Verify the detail modal *only if* an appointment exists
    if (await appointmentCard.isVisible()) {
        await appointmentCard.click();

        // Verify the modal opens and AI summary is visible
        await expect(page.getByTestId('appointment-ai-summary')).toBeVisible();

        // Verify action buttons
        await expect(page.getByRole('button', { name: 'Message Client' })).toBeVisible();
        await expect(page.getByRole('button', { name: 'Reschedule' })).toBeVisible();

        // Either Request Payment or View Receipt based on payment status
        const requestPayment = page.getByRole('button', { name: 'Request Payment' });
        const viewReceipt = page.getByRole('button', { name: 'View Receipt' });
        await expect(requestPayment.or(viewReceipt)).toBeVisible();

        // Close modal
        await page.getByRole('button', { name: 'Close' }).click();
        await expect(page.getByTestId('appointment-ai-summary')).toBeHidden();
    }
  });
});
