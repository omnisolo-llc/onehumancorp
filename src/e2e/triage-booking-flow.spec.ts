import { test, expect } from '@playwright/test';

test.describe('Agentic Booking Intake Flow', () => {
  test('Owner can approve a booking from a customer DM', async ({ page }) => {
    // 1. Navigate to the agent feed
    await page.goto('/feed');
    await expect(page).toHaveTitle(/Feed | One Human Corp/);

    // 2. Simulate an incoming booking inquiry
    const simulateBtn = page.getByTestId('simulate-booking-btn');
    await expect(simulateBtn).toBeVisible();
    await simulateBtn.click();

    // 3. Verify the pending booking appears in the feed
    // Wait for the new item to show up
    await page.waitForTimeout(1000);

    // 4. Check that the UI renders correctly
    // Look for "NEW BOOKING REQUEST" which is mapped to "Draft Booking" action_type
    const bookingCard = page.locator('.bg-white.dark\\:bg-\\[\\#1E1E1E\\]').filter({ hasText: 'NEW BOOKING REQUEST' }).first();
    await expect(bookingCard).toBeVisible();

    // Validate we see the simulated details
    await expect(bookingCard).toContainText('Repair Service');
    await expect(bookingCard).toContainText('Tuesday at 2:00 PM');
    await expect(bookingCard).toContainText('$150');

    // 5. Check action buttons
    const approveBtn = bookingCard.getByTestId('feed-approve-btn');
    await expect(approveBtn).toBeVisible();
    await expect(approveBtn).toHaveText(/Approve & Confirm/i);

    // Make sure touch target is at least 44x44
    const btnBox = await approveBtn.boundingBox();
    expect(btnBox?.width).toBeGreaterThanOrEqual(44);
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    // 6. Approve the booking
    await approveBtn.click();

    // Verify it disappears from the feed (because lifecycle_state changes to APPROVED)
    await expect(bookingCard).not.toBeVisible();
  });
});
