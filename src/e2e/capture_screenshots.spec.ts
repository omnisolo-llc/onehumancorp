import { test, expect } from '@playwright/test';

test('Capture Help Center screenshots', async ({ page, baseURL }) => {
  await page.goto(baseURL || 'http://localhost:3000/api-docs');

  // Wait for the help widget button and click it
  const helpButton = page.locator('button[aria-label="Help"]');
  await helpButton.waitFor({ state: 'visible' });
  await helpButton.click();

  // Wait for the widget container to be visible
  const widgetContainer = page.locator('#help-widget-container');
  await widgetContainer.waitFor({ state: 'visible' });

  // Take desktop screenshot
  await page.setViewportSize({ width: 1280, height: 800 });
  await page.screenshot({ path: 'test-results/desktop-help.png', fullPage: true });

  // Take mobile screenshot
  await page.setViewportSize({ width: 375, height: 812 });
  await page.screenshot({ path: 'test-results/mobile-help.png', fullPage: true });
});
