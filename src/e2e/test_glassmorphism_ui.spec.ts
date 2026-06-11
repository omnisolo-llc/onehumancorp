import { test, expect } from './fixtures';

test.describe('Glassmorphism UI Audit', () => {
  test('Verify setup page uses 16px border radius', async ({ page }) => {
    await page.goto('/setup');
    await page.waitForLoadState('networkidle');
    const container = page.locator('.container.glassmorphism').first();
    await expect(container).toBeVisible();
    const borderRadius = await container.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');
  });

  test('Verify input elements use 16px border radius', async ({ page }) => {
    await page.goto('/setup');
    await page.waitForLoadState('networkidle');
    const input = page.locator('input[type="text"]').first();
    await expect(input).toBeVisible();
    const borderRadius = await input.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');
  });

  test('Verify dashboard buttons use 16px border radius', async ({ page }) => {
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
    const button = page.locator('button').first();
    await expect(button).toBeVisible();
    const borderRadius = await button.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');
  });

  test('Verify POS buttons use 16px border radius', async ({ page }) => {
    await page.goto('/pos');
    await page.waitForLoadState('networkidle');
    const button = page.locator('.charge-btn').first();
    await expect(button).toBeVisible();
    const borderRadius = await button.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');
  });

  test('Verify Quote page containers use 16px border radius', async ({ page }) => {
    await page.goto('/quote');
    await page.waitForLoadState('networkidle');
    const container = page.locator('.glass-panel').first();
    await expect(container).toBeVisible();
    const borderRadius = await container.evaluate((el) => {
      return window.getComputedStyle(el).borderRadius;
    });
    expect(borderRadius).toBe('16px');
  });
});
