import { test, expect } from '@playwright/test';

test('Screens use skeleton loaders instead of circular spinners', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });

  await page.goto('http://localhost:8081');

  // Wait for the app to initialize
  await page.waitForTimeout(5000);

  // Click reload now if there's a new version banner blocking the app
  try {
      if (await page.locator('text=A new version is available!').isVisible({ timeout: 2000 })) {
          await page.locator('button:has-text("Reload Now")').click();
          await page.waitForTimeout(5000);
      }
  } catch (e) { }

  // Keyboard navigation initializes accessibility DOM in Flutter Web
  for (let i = 0; i < 5; i++) {
    await page.keyboard.press('Tab');
    await page.waitForTimeout(200);
  }

  // Click enable accessibility if it exists
  try {
      if (await page.locator('flt-semantics-placeholder[aria-label="Enable accessibility"]').isVisible({ timeout: 2000 })) {
          await page.locator('flt-semantics-placeholder[aria-label="Enable accessibility"]').click();
          await page.waitForTimeout(5000);
      }
  } catch (e) { }

  // Login flow via canvas clicks
  await page.mouse.click(187, 300); // Click somewhere near email input
  await page.waitForTimeout(500);
  await page.keyboard.type('test@example.com');

  await page.keyboard.press('Tab');
  await page.waitForTimeout(500);
  await page.keyboard.type('password123');

  await page.keyboard.press('Tab');
  await page.waitForTimeout(500);
  await page.keyboard.press('Enter');

  // Immediately capture the screenshot while loading is happening
  await page.waitForTimeout(500);
  await expect(page).toHaveScreenshot('dashboard_load.png', { maxDiffPixelRatio: 0.2 });
});
