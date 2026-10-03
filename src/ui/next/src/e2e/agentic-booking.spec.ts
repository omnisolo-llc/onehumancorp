import { test, expect } from '../../../../e2e/fixtures';

test.describe('Agentic Service Booking CUJ', () => {
  test('Customer requests a service and owner confirms the booking', async ({ page }) => {
    // Warm the owner cache before creating the booking. The reservation must
    // invalidate this snapshot instead of leaving the new approval invisible.
    const cachedFeed = await page.request.get('/api/v1/agent-feed');
    expect(cachedFeed.status()).toBe(200);

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
    const reservation = page.waitForResponse(response => response.url().includes('/api/v1/booking/engine/reserve') && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Confirm Booking' }).click();
    const reserved = await reservation;
    expect(reserved.status()).toBe(200);
    const { booking_id: bookingId } = await reserved.json();
    expect(bookingId).toBeTruthy();

    // Verify submission success
    const heading = page.getByRole('heading', { name: /Almost there!|Booking request confirmed\./i });
    await expect(heading.first()).toBeVisible({ timeout: 15000 });

    // 2. Owner Flow. The suite's default page already carries the owner session.
    await page.goto('/feed');

    const refreshedFeed = await page.request.get('/api/v1/agent-feed');
    expect(refreshedFeed.status()).toBe(200);
    const { items } = await refreshedFeed.json();
    const bookingItem = items.find((item: { proposed_action?: { booking_id?: string } }) => item.proposed_action?.booking_id === bookingId);
    expect(bookingItem).toBeDefined();
    const bookingCard = page.getByTestId('agent-feed-card').filter({ hasText: /booking request/i }).first();
    const approveBtn = bookingCard.getByRole('button', { name: /^Approve$/i });
    await expect(bookingCard).toBeVisible({ timeout: 5000 });
    await expect(approveBtn).toBeVisible();
    await approveBtn.click();
    await expect(bookingCard).toBeHidden({ timeout: 5000 });
  });
});
