import { test, expect } from '@playwright/test';

test.describe('Viral Share to Unlock - Digital Business Card', () => {

  test('Shows soft paywall and allows unlock via share', async ({ page }) => {
    // Navigate to the digital business card generator using relative path
    await page.goto('/digital-business-card');

    // Fill in basic details to see preview update
    await page.fill('input[placeholder="e.g. Jane Doe"]', 'Test User');

    // Check that "Powered by OHC" is visible in the preview initially
    await expect(page.locator('text=⚡ Powered by OHC')).toBeVisible();

    // Click the "Remove \'Powered by OHC\' branding" checkbox
    await page.click('text=Remove "Powered by OHC" branding');

    // The Soft Paywall should appear
    await expect(page.locator('text=Upgrade to Pro').first()).toBeVisible();
    await expect(page.locator('text=Share to Unlock for Free').first()).toBeVisible();

    // Wait for the modal animation
    await page.waitForTimeout(300);

    // Some tests fail because the locator might match multiple things or it opens a new page and the old one closes too fast.
    // Instead of waiting for a popup, we can stub window.open to prevent the actual navigation, which might break tests
    await page.evaluate(() => {
        window.open = () => null;
    });

    await page.click('text=Share to Unlock for Free');

    // The modal should close
    await expect(page.locator('text=Share to Unlock for Free')).not.toBeVisible();

    // The checkbox should be checked (verified by the branding being gone)
    await expect(page.locator('text=⚡ Powered by OHC')).not.toBeVisible();
  });
});
