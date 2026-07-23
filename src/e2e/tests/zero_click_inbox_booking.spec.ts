import { test, expect } from '@playwright/test';

test.describe('Zero-Click Lead to Booking Approval Flow', () => {
  test('Persona: Maya (Home Baker) approves an AI-drafted quote in the unified inbox', async ({ page, request }) => {
    // Navigate to the unified inbox
    await page.goto('/inbox');
    await page.setViewportSize({ width: 375, height: 812 });

    // We expect the 'Zero-Click Approve' button to be visible for a pending message.
    // Ensure the fallback has loaded the UI.
    await expect(page.locator('text=Zero-Click Approve').first()).toBeVisible({ timeout: 15000 });

    // Click the Zero-Click Approve button
    const approveBtn = page.locator('button', { hasText: '✨ Zero-Click Approve' }).first();
    await approveBtn.click();

    // We expect the button to no longer be visible (it should transition to APPROVED)
    await expect(approveBtn).toBeHidden({ timeout: 10000 });
  });
});
