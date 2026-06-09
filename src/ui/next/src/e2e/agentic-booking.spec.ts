import { test, expect } from '@playwright/test';

test.describe('Agentic Service Booking & Quoting CUJ', () => {
  test('Customer selects a slot and reserves it via calendar UI', async ({ page }) => {
    // 1. Customer Flow
    // Navigate to booking form
    await page.goto('/booking');

    // Check elements
    await expect(page.getByRole('heading', { name: 'Book an Appointment' })).toBeVisible();

    // Verify slots are loaded and visible for the default date (or we can select tomorrow if empty)
    // Wait for the slots grid to load
    await page.waitForTimeout(2000);

    // If slots are loaded, click the first available slot
    const firstSlot = page.locator('button:not(:disabled)').filter({ hasText: /:/ }).first();

    // We expect there to be at least one slot available
    await expect(firstSlot).toBeVisible({ timeout: 5000 });
    await firstSlot.click();

    // Submit form (Confirm & Pay Deposit)
    await page.getByRole('button', { name: 'Confirm & Pay Deposit' }).click();

    // The Stripe redirect happens normally here.
    // If it navigates to stripe we check URL, if it falls back to confirmation screen, check that.
    // Since Stripe is external, let's just make sure it navigated away from /booking or shows confirmation.
    await page.waitForTimeout(2000);

    try {
        await expect(page.getByRole('heading', { name: 'Booking Confirmed!' })).toBeVisible({ timeout: 3000 });
    } catch {
        // Must be stripe
        expect(page.url()).toContain('checkout.stripe.com');
    }
  });
});
