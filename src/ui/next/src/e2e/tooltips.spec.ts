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
    });

    test('renders settings tooltips on hover', async ({ page }) => {
        await page.goto('/settings');

        // Wait for the page to load
        await page.waitForLoadState('networkidle');

        // Verify the Delivery tooltip
        const deliveryToggle = page.locator('label', { hasText: 'Enable Local Delivery' });
        await expect(deliveryToggle).toBeVisible();

        await deliveryToggle.hover();

        // Wait for the tooltip text to appear
        const deliveryTooltipText = page.locator('div', { hasText: 'Turn this on to offer local delivery to your customers.' }).last();
        await expect(deliveryTooltipText).toBeVisible({ timeout: 5000 });

        // Move mouse away
        await page.mouse.move(0, 0);
    });
});
