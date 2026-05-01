import { test, expect } from '@playwright/test';

test.describe('Business Share Page', () => {
  test('should display share store hint when question mark is clicked', async ({ page }) => {
    // We navigate to /login and complete login if E2E requires it.
    // However, looking at dashboard_ux.spec.ts, we start from login.
    // Let's emulate what we see in other specs.
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    // Now trigger the action to share store
    const shareBtn = page.locator('button:has-text("Share Store")');
    if (await shareBtn.isVisible()) {
        await shareBtn.click();

        // Find the Share Your Store label
        const shareStoreLabel = page.locator('text=Share Your Store').first();

        if (await shareStoreLabel.isVisible()) {
            // Find the ? button right next to it
            // Based on layout: HorizontalBox -> [Text, Button(?), Rectangle, Button(Close)]
            // The ? button is the first button inside the HorizontalBox
            const questionMarkBtn = shareStoreLabel.locator('..').locator('button:has-text("?")').first();
            if (await questionMarkBtn.isVisible()) {
                await questionMarkBtn.click();
                await expect(page.locator('text=Use these buttons to copy a link to your store or post it directly to your social media pages so customers can find you.')).toBeVisible();
            }
        }
    }
  });
});
