import { test, expect } from './fixtures';

test.describe('WhatsApp Link Generator Growth Loop', () => {
    test('renders generator, handles input, shows link, and verifies branding', async ({ page }) => {
        await page.goto('/whatsapp-link-generator');

        // Verify the main heading
        await expect(page.locator('h1', { hasText: 'WhatsApp Link Generator 📱' }).first()).toBeVisible();

        // Check the "Powered by OHC" footer is present on the page
        const footerLink = page.locator('footer a', { hasText: '⚡ Powered by OHC' });
        await expect(footerLink).toBeVisible();

        // Check that "Get Link" is disabled initially
        const getLinkBtn = page.locator('button', { hasText: 'Get Link' });
        await expect(getLinkBtn).toBeDisabled();

        // Fill in phone number
        const phoneInput = page.getByLabel('WhatsApp Phone Number');
        await expect(phoneInput).toBeVisible();
        await phoneInput.fill('1234567890');

        // Fill in message
        const messageInput = page.getByLabel('Pre-filled Message');
        await expect(messageInput).toBeVisible();
        await messageInput.fill('Hi, I am interested in your services.');

        // "Get Link" should now be enabled
        await expect(getLinkBtn).toBeEnabled();

        // Verify the live preview updates with the text
        const previewText = page.locator('.whitespace-pre-wrap', { hasText: 'Hi, I am interested in your services.' });
        await expect(previewText).toBeVisible();
        await expect(previewText).toContainText('⚡ Powered by OHC');

        // Click "Get Link" to open modal
        await getLinkBtn.click();

        // Verify modal appears
        await expect(page.locator('h2', { hasText: 'Your WhatsApp Link' })).toBeVisible();

        // Verify the link has the branding encoded
        const linkTextarea = page.locator('textarea[readonly]');
        const generatedLink = await linkTextarea.inputValue();
        expect(generatedLink).toContain('wa.me/1234567890');
        expect(generatedLink).toContain(encodeURIComponent('Hi, I am interested in your services.\n\n⚡ Powered by OHC'));

        // Close modal
        await page.locator('button', { hasText: 'Close' }).click();

        // Try to remove branding and verify paywall
        const removeBrandingToggle = page.locator('label', { hasText: 'Remove "Powered by OHC" Badge (Pro)' });
        await removeBrandingToggle.click();

        // Verify paywall modal appears
        await expect(page.locator('h2', { hasText: 'Upgrade to Pro' })).toBeVisible();
    });
});
