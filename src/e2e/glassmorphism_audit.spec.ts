import { test, expect } from '@playwright/test';

test.describe('Glassmorphism Audit', () => {
  test('verify dashboard panels use ohc-hybrid-panel instead of mac-glass-container', async ({ page }) => {
    // Navigate to the dashboard
    await page.goto('http://localhost:3000/dashboard');

    // Wait for the page to load
    await expect(page.locator('text=Welcome back')).toBeVisible({ timeout: 15000 }).catch(() => null);

    // Verify no elements have 'mac-glass-container'
    const legacyElements = await page.locator('.mac-glass-container').count();
    expect(legacyElements).toBe(0);

    // Verify that at least some elements have 'ohc-hybrid-panel'
    const newElements = await page.locator('.ohc-hybrid-panel').count();
    expect(newElements).toBeGreaterThan(0);
  });
});
