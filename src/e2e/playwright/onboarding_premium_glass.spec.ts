import { test, expect } from '@playwright/test';

test.describe('Onboarding and Website Builder Premium Glass Compliance', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('onboarding wizard has translucent glass containers and correct touch targets', async ({ page }) => {
    // Navigate to the onboarding setup page
    await page.goto('http://127.0.0.1:18789/setup.html');
    await expect(page).toHaveTitle(/OneHumanCorp|OHC/);

    // Initial Screen
    await expect(page.locator('h1', { hasText: 'Tell us about your business' })).toBeVisible({ timeout: 15000 });

    // Verify glassmorphism container has border-radius 16px
    const glassContainer = page.locator('.glassmorphism').first();
    const borderRadius = await glassContainer.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');

    // Verify Generate Storefront button has min-height >= 44px
    const generateBtn = page.locator('#generate-storefront-btn');
    const box = await generateBtn.boundingBox();
    expect(box?.height).toBeGreaterThanOrEqual(44);

    // Verify Chat Send button has min-height >= 44px
    const sendBtn = page.locator('#chat-send-btn');
    const sendBox = await sendBtn.boundingBox();
    expect(sendBox?.height).toBeGreaterThanOrEqual(44);
  });
});
