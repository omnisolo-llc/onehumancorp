import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

currentAppSmoke('autonomous_booking');

test.describe('Agentic Service Booking & Rescheduling CUJ', () => {
  test('Owner sees Action Cards for Booking and Rescheduling', async ({ page }) => {
    // 1. Owner Flow
    // Login to application
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('carlos@ohc.test');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();

    // Verify successful login
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 2. Verify Agent Activity Action Card
    await expect(page.getByText('Agent Activity')).toBeVisible();
    await expect(page.getByText('Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed.')).toBeVisible();

    // 3. Verify Approval Action Card for Rescheduling
    await expect(page.getByText('Action Required')).toBeVisible();
    await expect(page.getByText('Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?')).toBeVisible();

    // 4. Verify Interactive Elements (Buttons)
    const approveBtn = page.getByRole('button', { name: 'Approve' });
    const editBtn = page.getByRole('button', { name: 'Edit' });
    const denyBtn = page.getByRole('button', { name: 'Deny' });

    await expect(approveBtn).toBeVisible();
    await expect(editBtn).toBeVisible();
    await expect(denyBtn).toBeVisible();

    // Assert that the buttons are not disabled (they should be clickable)
    await expect(approveBtn).toBeEnabled();
    await expect(editBtn).toBeEnabled();
    await expect(denyBtn).toBeEnabled();
  });
});
