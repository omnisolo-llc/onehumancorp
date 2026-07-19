import { test, expect } from '../../../../e2e/fixtures';


test.describe('Interactive Demo Generator Growth Loop', () => {
    test('generator links from somewhere or is accessible directly, generates an embed with a viral footer', async ({ page, adminUser, loginAs }) => {
        // 1. Log in
        await loginAs(page, adminUser);

        // 2. Navigate to interactive demo page
        await page.goto('/interactive-demo');

        // Verify we are on the generator page
        await expect(page.getByRole('heading', { name: 'Interactive Demo Generator' })).toBeVisible();

        // 4. Modify some config values
        const titleInput = page.locator('input[value="My Interactive Demo"]');
        await titleInput.fill('Test Product Showcase');

        const descInput = page.locator('textarea');
        await descInput.fill('See how it works in real time.');

        // 5. Verify the live preview updates
        await expect(page.locator('.bg-white.border.border-gray-200 > h3')).toHaveText('Test Product Showcase');
        await expect(page.locator('.bg-white.border.border-gray-200 > p')).toHaveText('See how it works in real time.');
        await expect(page.locator('.bg-white.border.border-gray-200')).toContainText('⚡ Powered by OHC');

        // 6. Verify the embed code contains the viral link and correct text
        const codeOutput = page.locator('textarea[readonly]');
        const embedHtml = await codeOutput.inputValue();
        expect(embedHtml).toContain('Test Product Showcase');
        expect(embedHtml).toContain('See how it works in real time.');
        expect(embedHtml).toContain('⚡ Powered by OHC');

        // 7. Test removing branding soft paywall
        // Ensure the user doesn't have pro
        await page.evaluate(() => { localStorage.setItem('has_pro', 'false'); window.dispatchEvent(new Event('storage')); });

        // Try to toggle "Remove branding"
        const toggleInput = page.locator('input[type="checkbox"]');
        // Click the label or sibling to trigger the click
        await toggleInput.locator('xpath=..').click();

        // Verify soft paywall modal is shown
        const modal = page.locator('text=Make the Interactive Demo 100% yours.').locator('xpath=../..');
        await expect(modal).toBeVisible();
        await expect(page.getByRole('heading', { name: 'Upgrade to Pro' })).toBeVisible();

        // Dismiss modal
        await page.locator('button:has-text("✕")').click();
        await expect(modal).not.toBeVisible();

        // 8. Test removing branding successfully as a Pro user
        await page.evaluate(() => { localStorage.setItem('has_pro', 'true'); window.dispatchEvent(new Event('storage')); });

        // The toggle should now work without modal
        await toggleInput.locator('xpath=..').click();
        await expect(modal).not.toBeVisible();

        // Watermark should be gone in preview
        await expect(page.locator('.bg-white.border.border-gray-200')).not.toContainText('⚡ Powered by OHC');

        // Watermark should be gone in code output
        const newEmbedHtml = await codeOutput.inputValue();
        expect(newEmbedHtml).not.toContain('⚡ Powered by OHC');
    });
});
