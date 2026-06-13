import { test, expect } from './fixtures';

test.describe('Glassmorphism UI Audit', () => {
  test('Verify dashboard glass panels use 16px border radius', async ({ page }) => {
    await page.goto('/dashboard');
    const container = page.locator('.app-panel').first();
    await expect(container).toBeAttached({ timeout: 15000 });
    const borderRadius = await container.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');
  });

  test('Verify dashboard buttons use 8px border radius', async ({ page }) => {
    await page.goto('/dashboard');
    const button = page.locator('.app-button').first();
    await expect(button).toBeAttached({ timeout: 15000 });
    const borderRadius = await button.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('8px');
  });
});
