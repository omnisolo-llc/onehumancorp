import { test, expect, devices } from '@playwright/test';

test.describe('Help Center Desktop & Mobile', () => {
  test('Verify help center on desktop', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.goto('http://localhost:3000/help');
    await page.waitForTimeout(1000);
    await page.screenshot({ path: 'help-center-desktop.png' });

    await page.goto('http://localhost:3000/help/getting-started');
    await page.waitForTimeout(1000);
    await page.screenshot({ path: 'help-article-desktop.png' });
  });

  test('Verify help center on mobile', async ({ page }) => {
    await page.setViewportSize(devices['iPhone 12'].viewport);
    await page.goto('http://localhost:3000/help');
    await page.waitForTimeout(1000);
    await page.screenshot({ path: 'help-center-mobile.png' });

    await page.goto('http://localhost:3000/help/getting-started');
    await page.waitForTimeout(1000);
    await page.screenshot({ path: 'help-article-mobile.png' });
  });
});
