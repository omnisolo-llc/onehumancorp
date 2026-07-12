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

        // Expect the new AgentUpsellPaywall modal to appear
        const modalHeader = page.locator('h2', { hasText: 'Unlock Pro Mode' });
        await expect(modalHeader).toBeVisible();
        await expect(page.locator('h3', { hasText: 'Viral Growth Offer' })).toBeVisible();

        // Click "Get 14 Days Free via Invite"
        const generateBtn = page.locator('button', { hasText: 'Get 14 Days Free via Invite' });
        await expect(generateBtn).toBeVisible();
        await generateBtn.click();

        // Expect it to generate a link and switch to the copy state
        const linkInput = page.locator('input[readonly]');
        await expect(linkInput).toBeVisible({ timeout: 10000 });
        await expect(linkInput).toHaveValue(/^https?:\/\//);

        // Click the WhatsApp share link (should unlock optimistically)
        const whatsappLink = page.locator('a', { hasText: 'Share on WhatsApp' });
        await expect(whatsappLink).toBeVisible();

        // Use evaluate to click so we don't actually navigate to whatsapp
        await whatsappLink.evaluate((node) => node.click());

        // The modal should close and the toggle should now be active
        await expect(modalHeader).not.toBeVisible();
        await expect(proModeToggle).toHaveAttribute('aria-pressed', 'true');
    });
});
