import { test, expect } from '@playwright/test';

test.describe('Chaos Report UI', () => {
  test('should render properly and handle dark mode toggle', async ({ page }) => {
    // Navigate to the Chaos Report page
    await page.goto('/chaos-report');

    // Verify header exists
    await expect(page.getByRole('heading', { name: 'System Reliability Report' })).toBeVisible();

    // Verify sections have the premium-glass class
    const firstSection = page.locator('section').first();
    await expect(firstSection).toHaveClass(/premium-glass/);

    // Initial state (assuming light mode defaults) or whatever system preference is.
    // Let's ensure the wrapper changes classes appropriately when toggled.
    const wrapperDiv = page.locator('.min-h-screen');
    const isInitiallyDark = await wrapperDiv.evaluate(el => el.classList.contains('dark'));

    // Find and click the toggle button
    const toggleButton = page.getByRole('button', { name: /Toggle .* Mode/ });
    await expect(toggleButton).toBeVisible();
    await toggleButton.click();

    // Verify the class changes on the wrapper
    const isNowDark = await wrapperDiv.evaluate(el => el.classList.contains('dark'));
    expect(isNowDark).toBe(!isInitiallyDark);

    // Toggle back
    await toggleButton.click();
    const isFinallyDark = await wrapperDiv.evaluate(el => el.classList.contains('dark'));
    expect(isFinallyDark).toBe(isInitiallyDark);
  });
});
