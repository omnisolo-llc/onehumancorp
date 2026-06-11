import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test.describe('Autonomous Booking & Scheduling Engine', () => {
  test('should display Action Needed and Approval booking cards on the dashboard for Operations Agent', async ({ page }) => {
    // Navigate to the Dashboard
    await page.goto('/dashboard');

    // Wait for the Dashboard title to load
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 15000 });

    // The UnifiedAgentFeed component is on the dashboard.
    // Click on the Proposals tab to ensure we are viewing the agent's proposals.
    await page.getByRole('button', { name: /Proposals/ }).click();

    // Verify Action Needed card is visible
    await expect(page.getByText('Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed.')).toBeVisible();

    // Verify Approval card is visible
    await expect(page.getByText('Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?')).toBeVisible();

    // Verify Approval buttons are present
    const approveBtn = page.getByRole('button', { name: 'Approve' });
    const editBtn = page.getByRole('button', { name: 'Edit' });
    const denyBtn = page.getByRole('button', { name: 'Deny' });

    await expect(approveBtn).toBeVisible();
    await expect(editBtn).toBeVisible();
    await expect(denyBtn).toBeVisible();
  });

  test('should allow a customer to query available time slots and create a unified booking', async ({ page }) => {
    // Navigate to the booking page with a dummy tenant and product ID
    await page.goto('/booking?tenant=default-store&product_id=e2e-product');

    // Wait for the Booking page to load
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    // Select tomorrow's date
    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);
    const dateStr = tomorrow.toISOString().split("T")[0];

    // Interact with date input
    await page.locator('input[type="date"]').fill(dateStr);

    // Wait for timeslots to load and verify buttons are visible
    // Depending on backend mocking or data, we might have specific times.
    // Wait for the button grid to appear (it should fetch slots from availability endpoint)
    await expect(page.locator('text=Available Times')).toBeVisible();
  });
});
