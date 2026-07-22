import { test, expect } from '@playwright/test';

test.describe('Website Builder Premium Glass Compliance', () => {
  test.use({ viewport: { width: 375, height: 667 } }); // strictly mobile viewport

  test('website builder has correct touch targets on primary buttons', async ({ page }) => {
    // Navigate to the website builder setup page
    await page.goto('http://127.0.0.1:18789/builder.html');

    // Wait for the main glassmorphism container
    const glassContainer = page.locator('.glassmorphism').first();
    await expect(glassContainer).toBeVisible({ timeout: 15000 });

    const borderRadius = await glassContainer.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');

    // Test the launch button on mobile
    const launchBtn = page.locator('#launch-btn');
    if (await launchBtn.isVisible()) {
       const box = await launchBtn.boundingBox();
       expect(box?.height).toBeGreaterThanOrEqual(44);
    }
  });
});
