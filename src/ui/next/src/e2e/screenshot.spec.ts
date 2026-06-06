import { test, expect } from '@playwright/test';

test('take screenshots of agent roster themes', async ({ page }) => {
  await page.goto('http://localhost:3000/agent-roster');

  // Set window size for standard view
  await page.setViewportSize({ width: 1280, height: 800 });

  // Wait for load
  await page.waitForTimeout(1000);

  // Take glass screenshot
  await page.click('button:nth-child(1)'); // Assume first button is glass
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'glass-theme.png', fullPage: true });

  // Take dark screenshot
  await page.click('button:nth-child(2)'); // Assume second button is dark
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'dark-theme.png', fullPage: true });

  // Take light screenshot
  await page.click('button:nth-child(3)'); // Assume third button is light
  await page.waitForTimeout(500);
  await page.screenshot({ path: 'light-theme.png', fullPage: true });
});
