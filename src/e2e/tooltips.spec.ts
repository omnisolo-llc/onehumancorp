import { test, expect } from './fixtures';

test.describe('Tooltip Registry', () => {
  test('should display tooltips on hover', async ({ page }) => {
    await page.goto('/dashboard');

    // Find an element that should have a tooltip (e.g. the team activity section)
    const teamActivityElement = page.getByText('Team Activity');
    await expect(teamActivityElement).toBeVisible();

    // Hover over the element to trigger the tooltip
    await teamActivityElement.hover();

    // The tooltip is rendered in a portal, wait for it to appear
    const tooltip = page.locator('.animate-fade-in-up');
    await expect(tooltip).toBeVisible({ timeout: 5000 });

    // Check that it contains the expected text
    await expect(tooltip).toContainText('See exactly what your AI helpers are doing right now.');
  });
});
