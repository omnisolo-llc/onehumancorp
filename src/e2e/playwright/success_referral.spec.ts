import { test, expect } from './fixtures';

test.describe('Success Referral Card E2E', () => {
  test('verify referral success card appears after booking natively', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Sign in' }).click();

    await page.waitForURL('**/dashboard**');
    await page.waitForLoadState('networkidle');

    // In a real CUJ, a user gets to the success page after an action, like a booking deposit.
    // There is an e2e seed triage action that tentatively booked a roof repair estimate.
    // It's under Operations tab. If tabs don't exist, we look directly.
    const anyTriageCard = page.locator('.triage-item').first();
    await expect(anyTriageCard).toBeVisible({ timeout: 15000 });

    const approveBtn = page.getByTestId('approve-btn').first();
    await expect(approveBtn).toBeVisible({ timeout: 15000 });
    await approveBtn.click();

    await page.waitForLoadState('networkidle');

    // Wait for the success page elements natively.
    // Some implementations just show the referral card inline after success, others navigate.
    // We check that the referral success card appears
    const card = page.getByTestId('referral-success-card');
    await expect(card).toBeVisible({ timeout: 15000 });
    await expect(card).toContainText('Start Your AI Business');
  });
});
