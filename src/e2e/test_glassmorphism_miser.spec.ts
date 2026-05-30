import { test, expect } from './fixtures';

test.describe('Premium Glassmorphism Aesthetic on Cost Dashboard', () => {
  test('should display glassmorphism UI correctly', async ({ page }) => {
    await page.goto('/cost-dashboard');
    const header = page.locator('header').first();
    const style = await header.evaluate((el) => window.getComputedStyle(el).backdropFilter);
    expect(style).toContain('blur(30px)');
  });
});
