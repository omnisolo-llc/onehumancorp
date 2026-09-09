import { test, expect } from '../../../../e2e/fixtures';

test.describe('Agentic Service Booking CUJ', () => {
  test('Customer requests a service and owner confirms the booking', async ({ page }) => {
    // 1. Customer Flow
    // Navigate to booking form
    await page.goto('/booking?tenant=e2e-tenant&service_id=e2e-product-class');

    // Check elements
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    // Fill form
    await page.getByPlaceholder('Jane Doe').fill('John Doe');
    await page.getByPlaceholder('jane@example.com').fill('johndoe@example.com');
    const nextDay = new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString().split('T')[0];
    await page.locator('input[type="date"]').fill(nextDay);

    // Select first slot
    const firstSlot = page.getByRole('button', { name: /\d{1,2}:\d{2}/ }).first();
    await expect(firstSlot).toBeVisible({ timeout: 10000 });
    await firstSlot.click();

    await page.getByPlaceholder('What do you need help with?').fill('I need help fixing a leaky pipe in my kitchen sink.');

    // Submit form
    await page.getByRole('button', { name: 'Confirm Booking' }).click();

    // Verify submission success
    const heading = page.getByRole('heading', { name: /Almost there!|Booking request confirmed\./i });
    await expect(heading.first()).toBeVisible({ timeout: 15000 });

    // 2. Owner Flow. The suite's default page already carries the owner session.
    await page.goto('/feed');

    const bookingCard = page.getByTestId('agent-feed-card').filter({ hasText: /booking request/i }).first();
    const approveBtn = bookingCard.getByRole('button', { name: /^Approve$/i });
    await expect(bookingCard).toBeVisible({ timeout: 5000 });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
    await expect(bookingCard).toBeHidden({ timeout: 5000 });
  });
});
