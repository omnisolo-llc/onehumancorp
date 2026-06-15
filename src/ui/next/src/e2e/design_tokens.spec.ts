import { test, expect } from '@playwright/test';

test.describe('OHC Design Token Compliance', () => {
  test('buttons have 8px border-radius', async ({ page }) => {
    await page.goto('/dashboard');
    // Wait for the page to load
    await page.waitForSelector('.app-button', { timeout: 10000 });

    // Evaluate the computed style of the first button
    const borderRadius = await page.$eval('.app-button', (el) => {
      return window.getComputedStyle(el).borderRadius;
    });

    expect(borderRadius).toBe('8px');
  });

  test('primary buttons use Apple Blue accent', async ({ page }) => {
    await page.goto('/dashboard');
    // Wait for the page to load
    await page.waitForSelector('.app-button.primary', { timeout: 10000 });

    // Evaluate the computed style of the primary button
    const bgColor = await page.$eval('.app-button.primary', (el) => {
      return window.getComputedStyle(el).backgroundColor;
    });

    // #0071E3 is rgb(0, 113, 227)
    expect(bgColor).toBe('rgb(0, 113, 227)');
  });
});
