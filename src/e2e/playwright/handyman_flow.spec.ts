import { test, expect } from '@playwright/test';

test.describe('Agentic Field Service Scheduling & Quoting', () => {
    test('CUJ: Handyman quote draft and tentative booking flow', async ({ page }) => {
        // Go to feed directly
        await page.goto('/feed');
        await page.waitForLoadState('networkidle');

        await expect(page.getByTestId('simulate-booking-btn')).toBeVisible({ timeout: 15000 });
        await page.getByTestId('simulate-booking-btn').click();

        // Wait for the quote card to appear. It should contain our text.
        await expect(page.locator('text=Action Required: Approve Estimate').first()).toBeVisible({ timeout: 15000 });
        await expect(page.locator('text=My sink is leaking, can you come today?').first()).toBeVisible();

        // Click "Approve"
        const approveBtn = page.getByTestId('approve-action-btn').first();
        await approveBtn.click();

        // Wait for it to disappear or show a success state.
        await expect(page.getByTestId('approve-action-btn').first()).toBeHidden({ timeout: 10000 });
    });
});
