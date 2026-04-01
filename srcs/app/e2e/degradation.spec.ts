import { test, expect } from '@playwright/test';

test.describe('Hybrid Architecture Degradation Verification', () => {
  test('gracefully handles offline/dropped connection without crashing', async ({ page }) => {
    // Navigate to the dashboard.
    await page.goto('/dashboard');

    // Wait for the app to initialize.
    await page.waitForFunction(() => {
      return document.querySelector('flt-glass-pane') !== null || document.querySelector('canvas') !== null;
    }, { timeout: 30000 });

    // Simulate network latency/offline mode.
    await page.route('**/*', route => {
      // Abort all API calls to simulate backend failure in Thin Client mode.
      if (route.request().url().includes('/api/')) {
        return route.abort('internetdisconnected');
      }
      return route.continue();
    });

    // Attempt an interaction that requires the backend.
    await page.keyboard.press('Tab');
    await page.keyboard.press('Enter');

    // Wait to ensure the app hasn't crashed.
    await page.waitForTimeout(2000);

    // Verify the canvas or flutter app is still present in the DOM (did not white-screen/crash).
    const flutterPresent = await page.evaluate(() => {
      return document.querySelector('flt-glass-pane') !== null || document.querySelector('canvas') !== null;
    });
    expect(flutterPresent).toBe(true);
  });
});
