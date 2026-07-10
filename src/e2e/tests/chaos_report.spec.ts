import { test, expect } from '@playwright/test';

test.describe('Chaos Report UX & Interaction', () => {
  test('Chaos Report loads correctly and toggles dark mode', async ({ page }) => {
    // Navigate to the chaos report page
    await page.goto('/chaos-report');

    // Verify title and structure
    await expect(page.locator('h1', { hasText: 'System Reliability Report' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Chaos Resilience Metrics' })).toBeVisible();

    // Verify premium-glass class is applied to the sections
    const sections = page.locator('section');
    await expect(sections.first()).toHaveClass(/premium-glass/);
    await expect(sections.nth(1)).toHaveClass(/premium-glass/);

    // Test dark mode toggle button
    const toggleButton = page.locator('button', { hasText: /Toggle (Dark|Light) Mode/ });
    await expect(toggleButton).toBeVisible();

    const initialClass = await page.locator('div').first().getAttribute('class');

    // Click toggle button
    await toggleButton.click();

    // The class should change to include 'dark' or change from dark to light
    const changedClass = await page.locator('div').first().getAttribute('class');
    expect(initialClass).not.toEqual(changedClass);
  });
});
