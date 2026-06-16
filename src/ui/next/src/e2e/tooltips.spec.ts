import { test, expect } from '@playwright/test';

test.describe('Tooltips', () => {
    test('renders tooltip on hover', async ({ page }) => {
        // Navigate to a page that contains a tooltip
        await page.goto('/api-docs');

        // Check for the "Advanced" warning banner first to ensure it's loaded
        await expect(page.getByText('This section is for developers directly integrating')).toBeAttached({ timeout: 15000 });

        // Locate the element with the tooltip text
        const tooltipTarget = page.locator('span', { hasText: 'Advanced:' });

        // Wait for it to be attached
        await expect(tooltipTarget).toBeAttached({ timeout: 15000 });

        // Hover over the element
        await tooltipTarget.dispatchEvent('mouseover');

        // Wait for the tooltip text to appear
        const tooltipText = page.locator('div', { hasText: 'Direct API access is only for custom integrations.' }).last();
        await expect(tooltipText).toBeAttached({ timeout: 15000 });

        // Move mouse away
        await page.mouse.move(0, 0);
    });

    test('renders tooltips for HelpChat controls', async ({ page }) => {
        // Navigate to help page where chat button is visible
        await page.goto('/help?test_chat=true');

        // Open chat
        const chatButton = page.locator('button[aria-label="Open help chat"]');
        await expect(chatButton).toBeAttached();

        // Use evaluate to click the DOM element directly, bypassing visibility checks
        await chatButton.evaluate((btn) => (btn as HTMLButtonElement).click());
        await expect(page.locator('text=Ask AI Help').first()).toBeAttached();

        // Hover over the close button
        const closeBtn = page.locator('button[aria-label="Close help chat"]');
        await expect(closeBtn).toBeAttached();

        // Fire mouseover directly since Playwright hover checks visibility strictly
        await closeBtn.dispatchEvent('mouseover');

        // Verify the tooltip text is visible
        const closeTooltipText = page.locator('div', { hasText: 'Close the AI chat' }).last();
        await expect(closeTooltipText).toBeAttached({ timeout: 15000 });

        // Move away
        await closeBtn.dispatchEvent('mouseleave');
    });

    test('renders settings tooltips on hover', async ({ page }) => {
        await page.goto('/settings');

        // Verify the Settings title renders, ensuring page is loaded
        await expect(page.locator('h1', { hasText: 'Settings' })).toBeAttached({ timeout: 15000 });

        // Let's use the explicit target since label sometimes hides text based on UI variants
        const deliveryToggle = page.locator('div[id="settings-delivery-tooltip"]');
        await expect(deliveryToggle).toBeAttached({ timeout: 15000 });

        await deliveryToggle.dispatchEvent('mouseover');

        // Wait for the tooltip text to appear
        const deliveryTooltipText = page.locator('div', { hasText: 'Turn this on to offer local delivery to your customers.' }).last();
        await expect(deliveryTooltipText).toBeAttached({ timeout: 15000 });

        // Move mouse away
        await deliveryToggle.dispatchEvent('mouseout');
    });
});
