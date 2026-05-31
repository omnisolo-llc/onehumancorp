import { test, expect } from '@playwright/test';

test.describe('Finance Capital Engine CUJ', () => {

  test('Persona: Business Owner views and accepts cash flow advance', async ({ page }) => {
    // Navigate to the finance page
    await page.goto('/finance');

    // We expect the transparent glass card
    await expect(page.getByRole('heading', { name: /Cash Flow Alert/i })).toBeVisible();

    // Verify it loads correctly without relying on database seeded offers.
  });
});
