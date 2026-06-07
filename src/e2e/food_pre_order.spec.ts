import { test, expect } from '@playwright/test';

test.describe('Food Pre-Order Flow', () => {
  test('Fatima can view the KDS and toggle language', async ({ page }) => {
    // Navigate to KDS Page
    await page.goto('/pos/kds');
    await page.evaluate(() => localStorage.clear());
    await page.reload();

    // Verify basic UI elements
    await expect(page.locator('text=Kitchen Display System')).toBeVisible();

    // The rest relies on proper seeding. For now we assert the core UI is accessible without mocked routes.
    // Ensure the language toggle is visible.
    await expect(page.getByTestId('lang-toggle')).toBeVisible();

    // Switch to Arabic
    await page.getByTestId('lang-toggle').click();

    // The header text should be conditionally updated or we should see AR active.
    // For now we check that the switch works and doesn't crash the app.
    const isArabic = await page.getByTestId('lang-toggle').innerText();
    expect(isArabic).toContain('Switch to EN');
  });
});
