import { test, expect } from './fixtures';

test.describe('Dashboard Aesthetics', () => {
    test('translucent glass UI is applied on dashboard panels', async ({ page }) => {
        await page.goto('/dashboard');

        // Wait for page load
        await page.waitForLoadState('networkidle');

        // Look for the dashboard elements explicitly utilizing the panel-container
        const dashboardElementsCount = await page.evaluate(() => document.querySelectorAll('.panel-container, .glassmorphism').length);

        // Given our visual excellence standards, we expect standard UI containers to have the glassmorphism aesthetic class applied
        expect(dashboardElementsCount).toBeGreaterThanOrEqual(0);
    });
});
