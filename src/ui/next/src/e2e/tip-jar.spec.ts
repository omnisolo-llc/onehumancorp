import { test, expect } from '../../../../e2e/fixtures';

test.describe('Tip Jar Widget Growth Loop', () => {
    test('renders the builder and viral branding loop correctly', async ({ page }) => {
        await page.goto('/tip-jar');

        // Verify the Builder UI loads
        await expect(page.locator('h1', { hasText: 'Tip Jar Builder' })).toBeVisible();

        // 1. Verify "Powered by OHC" watermark is visible in the preview
        const watermark = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(watermark).toBeVisible();
        await expect(watermark).toHaveAttribute('href', /\/api\/v1\/growth\/referrals\/click\?target=\/onboarding/);

        // 2. Interact with the configuration
        const nameInput = page.locator('input[placeholder="e.g. Creator Name"]');
        await nameInput.fill('Awesome Creator');
        await expect(page.locator('h3', { hasText: 'Awesome Creator' })).toBeVisible();

        // 3. Test the "Remove Branding" toggle triggering the soft paywall
        const checkboxLabel = page.locator('text=Remove "Powered by OHC" Badge');
        await expect(checkboxLabel).toBeVisible();

        // Ensure paywall modal is not visible initially
        await expect(page.locator('h2', { hasText: 'Upgrade to Pro' })).not.toBeVisible();

        // Click to remove branding
        await checkboxLabel.click();

        // Verify soft paywall modal appears
        const paywallHeader = page.locator('h2', { hasText: 'Upgrade to Pro' });
        await expect(paywallHeader).toBeVisible();
        await expect(page.locator('text=Make the Tip Jar 100% yours')).toBeVisible();

        // Close the modal
        await page.locator('button', { hasText: '×' }).click();
        await expect(paywallHeader).not.toBeVisible();

        // 4. Verify getting the embed code works
        const getWidgetBtn = page.locator('button', { hasText: 'Get Widget Code' });
        await getWidgetBtn.click();

        // Check if embed modal opens
        await expect(page.locator('h2', { hasText: 'Embed Tip Jar' })).toBeVisible();
        const textarea = page.locator('textarea[readonly]');
        await expect(textarea).toBeVisible();

        const embedCode = await textarea.inputValue();
        expect(embedCode).toContain('iframe');
        expect(embedCode).toContain('amounts=5%2C%2010%2C%2020');
        expect(embedCode).toContain('name=Awesome%20Creator');
    });
});
