import { test, expect } from '@playwright/test';

test.describe('Tooltips', () => {
    test('renders tooltip correctly over Advanced text on API docs page', async ({ page }) => {
        // Navigate to the API Docs page where the "Advanced" tooltip is present
        await page.goto('/api-docs');

        // Locate the "Advanced:" text that triggers the tooltip
        const advancedText = page.getByText('Advanced:');

        // Ensure the element exists and is visible
        await expect(advancedText).toBeVisible();

        // Hover over the text to trigger the tooltip
        await advancedText.hover();

        // Check if the tooltip text is visible
        const tooltip = page.getByText('Direct API access is only for custom integrations.');
        await expect(tooltip).toBeVisible();
    });
});
