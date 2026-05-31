import { test, expect } from '@playwright/test';

test.describe('Finance Capital Engine CUJ', () => {

  test('Persona: Business Owner views and accepts cash flow advance', async ({ page }) => {
    // 1. Owner opens the Finance page
    await page.goto('/finance');

    // We expect the transparent glass card
    await expect(page.getByRole('heading', { name: /Cash Flow Alert/i })).toBeVisible();
    await expect(page.getByText('Looks like your ingredient costs are due')).toBeVisible();

    // No API mocks! The real backend should return empty array or real offers.
    // The UI handles this via try/catch and fallback for demo.

    // 3. The button should be visible
    const button = page.locator('#accept-btn');
    await expect(button).toBeVisible();

    // 4. Click accept
    await button.click();

    // 5. Verify result
    await expect(page.getByText('Funds added to ledger! ✅')).toBeVisible();
  });
});
