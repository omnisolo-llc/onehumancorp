import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('autonomous_booking smoke', async ({ page, request }) => { await currentAppSmoke(page, request, 'autonomous_booking'); });

test.describe('Autonomous Booking & Scheduling Engine', () => {
  test('should display Action Needed and Approval booking cards on the dashboard for Operations Agent', async ({ page }) => {
    // We are testing the CUJ where the owner checks the Unified Agent Feed for
    // the Action/Approval cards after a customer negotiates and books.

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
});
