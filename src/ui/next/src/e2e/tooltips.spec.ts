import { test, expect } from '@playwright/test';

test.describe('Tooltips', () => {
    test('renders tooltip on hover', async ({ page }) => {
        // Navigate to a page that contains a tooltip
        await page.goto('/api-docs');

        // Locate the element with the tooltip text
        const tooltipTarget = page.locator('span', { hasText: 'Advanced:' });

        // Wait for it to be visible
        await expect(tooltipTarget).toBeVisible();

        // Hover over the element
        await tooltipTarget.hover();

        // Wait for the tooltip text to appear
        const tooltipText = page.locator('div', { hasText: 'Direct API access is only for custom integrations.' }).last();
        await expect(tooltipText).toBeVisible({ timeout: 5000 });

        // Move mouse away
        await page.mouse.move(0, 0);

        // Ensure it disappears or we are done
    });
});
