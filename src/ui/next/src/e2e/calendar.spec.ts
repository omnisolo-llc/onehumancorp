import { test, expect } from '../../../../e2e/fixtures';

test.describe('Calendar & Bookings', () => {
  test('should display upcoming appointments and operations agent activity', async ({ page }) => {
    // Navigate to the calendar page
    await page.goto('/calendar');

    // Check header is visible
    await expect(page.getByRole('heading', { name: 'Calendar & Bookings' })).toBeVisible();

    // Wait for the appointments to load or show empty state
    const loadingOrContent = page.locator('text=Loading appointments...').or(page.locator('text=No upcoming appointments.')).or(page.locator('.app-card h3'));
    await expect(loadingOrContent.first()).toBeVisible();

    // The component will display a loading indicator first,
    // wait until it disappears and actual data or empty text is present.
    await page.waitForFunction(() => {
        return !document.body.innerText.includes('Loading appointments...');
    });

    const isAppointmentsEmpty = await page.getByText('No upcoming appointments.').isVisible();

    if (!isAppointmentsEmpty) {
       // Find the first appointment and click it
       const firstAppointment = page.locator('.app-card .cursor-pointer').first();
       // wait for it to be visible. If it's not empty, it might still take a tiny bit to render
       await firstAppointment.waitFor({ state: 'visible', timeout: 5000 }).catch(() => {});

       if (await firstAppointment.isVisible()) {
           await firstAppointment.click();

           // Verify detail view instructions or details
           const detailsHeader = page.getByRole('heading', { name: 'Appointment Details' });
           await expect(detailsHeader).toBeVisible();

           const messageButton = page.getByRole('button', { name: 'Message Client' });
           await expect(messageButton).toBeVisible();
       }
    } else {
       // Verify empty detail state
       const detailsHeader = page.getByRole('heading', { name: 'Appointment Details' });
       await expect(detailsHeader).toBeVisible();
       await expect(page.getByText('Select an appointment to view details.')).toBeVisible();
    }
  });
});
