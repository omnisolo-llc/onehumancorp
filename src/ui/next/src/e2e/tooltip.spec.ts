import { test, expect } from '@playwright/test';

test.describe('TooltipRegistry', () => {
    test('renders tooltip on hover', async ({ page }) => {
        // Go to the dashboard, which has the Walkthrough 'Start Tour' button wrapped in a tooltip
        await page.goto('/dashboard');

        // Locate the Start Tour button
        const startTourButton = page.locator('button', { hasText: 'Start Tour' });
        await expect(startTourButton).toBeVisible();

        // The button is wrapped in a WithTooltip wrapper div.
        // We hover over the wrapper. The wrapper is the direct parent or the element itself if it catches events.
        const tooltipWrapper = startTourButton.locator('..');

        // Hover over the wrapper to trigger the tooltip
        await tooltipWrapper.hover();

        // The tooltip is rendered in a Portal or fixed absolute div, we wait for it to appear
        // Tooltip default API response text for walkthrough-btn-tooltip is:
        // "Start an interactive guide to learn how to use OHC."
        const tooltip = page.locator('div.fixed.z-\\[100\\]', { hasText: 'Start an interactive guide to learn how to use OHC.' });
        await expect(tooltip).toBeVisible();

        // Move mouse away to hide
        await page.mouse.move(0, 0);

        // Tooltip should disappear
        await expect(tooltip).not.toBeVisible();
    });
});
