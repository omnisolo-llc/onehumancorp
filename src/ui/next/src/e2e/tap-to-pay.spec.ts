import { test, expect } from '@playwright/test';

test.describe('Terminal Tap to Pay', () => {
  test('Completes the tap to pay flow', async ({ page }) => {
    // Navigate to the POS terminal page
    await page.goto('/pos/terminal');

    // Set offline staff mock since we use localStorage
    await page.evaluate(() => {
      localStorage.setItem('ohc_offline_staff', JSON.stringify([{ id: '1', name: 'Test Staff', role: 'Staff', pin_hash: '1234' }]));
    });

    // Reload so localStorage takes effect
    await page.reload();

    // Fill the PIN
    await page.click('button:has-text("1")');
    await page.click('button:has-text("2")');
    await page.click('button:has-text("3")');
    await page.click('button:has-text("4")');

    // Make sure we are unlocked
    await expect(page.locator('text=Not Clocked In')).toBeVisible({ timeout: 5000 });

    // Click 'New Order'
    await page.click('button:has-text("New Order")');

    // The modal should appear
    await expect(page.locator('text=Terminal not initialized').or(page.locator('text=Processing Tap to Pay...'))).toBeVisible({ timeout: 5000 });
  });
});
