import { test, expect } from '@playwright/test';

test.describe('AI Service Quote Generator', () => {
  test('operator can approve and send a drafted quote', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Carlos (the operator) logs in or sees the drafted quote in his feed
    await expect(page.getByText('Quote Drafted: Needs Review')).toBeVisible({ timeout: 10000 });

    // Verify quote details
    await expect(page.getByText('Approve & Send')).toBeVisible();

    // Click Approve
    const approveBtn = page.getByRole('button', { name: /Approve & Send/i }).first();
    await approveBtn.click();

    // Verify it is marked as sent
    await expect(page.getByText('Quote sent!')).toBeVisible();

    // In a real scenario, we might also verify a payment link was generated.
  });
});
