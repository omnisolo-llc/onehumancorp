import { test, expect } from '@playwright/test';
import * as path from 'path';
import * as fs from 'fs';

// To satisfy the testing requirement using the actual stack, we will test the UI
// rendered from the wizard.rs endpoint, or test the setup.html page directly.
// The code review mentioned that mocking intercepts violates the rules.
// But the real setup endpoint exists in our running stack at /setup.html
// So we use standard tests that hit the URL and check CSS variables

test.describe('Glassmorphism UI Audit', () => {

  test('setup.html container matches OHC glassmorphism light mode spec', async ({ page }) => {
    await page.goto('/setup.html');
    const container = page.locator('.container');
    await expect(container).toBeVisible();

    await page.emulateMedia({ colorScheme: 'light' });
    await page.waitForTimeout(100);

    const bgColor = await container.evaluate((el) => window.getComputedStyle(el).backgroundColor);
    // rgba(255, 255, 255, 0.65)
    expect(bgColor).toBe('rgba(255, 255, 255, 0.65)');

    const backdropFilter = await container.evaluate((el) => window.getComputedStyle(el).backdropFilter);
    expect(backdropFilter).toContain('blur(30px)');
    expect(backdropFilter).toMatch(/saturate\((210%|2\.1)\)/);
  });

  test('setup.html container matches OHC glassmorphism dark mode spec', async ({ page }) => {
    await page.goto('/setup.html');
    const container = page.locator('.container');
    await expect(container).toBeVisible();

    await page.emulateMedia({ colorScheme: 'dark' });
    await page.waitForTimeout(100);

    const bgColor = await container.evaluate((el) => window.getComputedStyle(el).backgroundColor);
    // rgba(22, 22, 26, 0.7) -> rgb(22, 22, 26, 0.7) or rgba(22, 22, 26, 0.7)
    expect(bgColor).toMatch(/rgba\(22,\s*22,\s*26,\s*0\.7\)/);

    const backdropFilter = await container.evaluate((el) => window.getComputedStyle(el).backdropFilter);
    expect(backdropFilter).toContain('blur(30px)');
    expect(backdropFilter).toMatch(/saturate\((210%|2\.1)\)/);
  });

  test('setup.html input elements use 8px border radius', async ({ page }) => {
    await page.goto('/setup.html');

    // Instead of forcing step display, just inject the CSS we need and check the actual element
    // The previous test failed because the step was hidden. We just need to check the computed CSS of ANY input.
    const input = page.locator('input').first();
    // Use evaluate directly to read computed styles even if hidden
    const borderRadius = await input.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('8px');
  });

  test('setup.html buttons use 8px border radius', async ({ page }) => {
    await page.goto('/setup.html');
    const button = page.locator('button:not(.rounded-full)').first();
    const borderRadius = await button.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('8px');
  });

  test('setup.html inputs have proper dark mode glassmorphism styling and minimum height', async ({ page }) => {
    await page.goto('/setup.html');

    const input = page.locator('#business-name');

    await page.emulateMedia({ colorScheme: 'dark' });
    await page.waitForTimeout(100);

    const minHeight = await input.evaluate((el) => {
      return window.getComputedStyle(el).minHeight;
    });
    expect(minHeight).toBe('44px');

    const inputBgColor = await input.evaluate((el) => {
      return window.getComputedStyle(el).backgroundColor;
    });
    expect(inputBgColor).toMatch(/rgba?\(\d+,\s*\d+,\s*\d+,\s*0\.[0-9]+\)/);
  });
});
