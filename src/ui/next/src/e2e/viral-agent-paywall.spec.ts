import { test, expect } from '../../../../e2e/fixtures';

test.describe('Viral Agent Paywall Growth Loop', () => {
    test('intercepts advanced skill toggle and displays trial extension offer', async ({ page }) => {
        // Go to agents page
        await page.goto('/agents');

        // Look for the "Pro Mode" toggle button in the header
        const proModeToggle = page.locator('button[aria-label="Toggle Pro Mode"]');
        await expect(proModeToggle).toBeVisible();

        // Check it's off by default (or set it off if it isn't, but our fixture should start it off via state)
        await expect(proModeToggle).toHaveAttribute('aria-pressed', 'false');

        // Click it to trigger the paywall
        await proModeToggle.click();

        // Expect the paywall modal to appear
        const modalHeader = page.locator('h2', { hasText: 'Upgrade to Pro' });
        await expect(modalHeader).toBeVisible();

        // Click "Share on X to activate Pro"
        const shareBtn = page.locator('button', { hasText: 'Share on X to activate Pro' });
        await expect(shareBtn).toBeVisible();
        await shareBtn.click();

        // The modal should close and the toggle should now be active
        await expect(modalHeader).not.toBeVisible();
        await expect(proModeToggle).toHaveAttribute('aria-pressed', 'true');
    });
});
