import { test, expect } from '@playwright/test';

test.describe('Viral Waitlist Generator', () => {
    test('navigates to dashboard, opens widget, configures and checks paywall', async ({ page }) => {
        // Go to dashboard
        await page.goto('/dashboard.html');

        // Check if the link exists
        const link = page.locator('#viral-waitlist-link');
        await expect(link).toBeVisible();

        // Click the link and wait for navigation
        await link.click();
        await page.waitForURL('**/viral-waitlist-generator.html');

        // Verify the title
        await expect(page.locator('h1.font-outfit')).toHaveText('Viral Waitlist Generator');

        // Configure the widget
        const productNameInput = page.locator('#product-name');
        await productNameInput.fill('Awesome New Gadget');

        const goalInput = page.locator('#referral-goal');
        await goalInput.fill('5');

        // Check live preview
        await expect(page.locator('#preview-title')).toHaveText('Join the Awesome New Gadget Waitlist');
        await expect(page.locator('#preview-desc')).toHaveText('Be the first to access our new launch. Refer 5 friends to jump to the front of the line!');

        // Check theme toggle
        const darkThemeBtn = page.locator('#theme-dark');
        await darkThemeBtn.click();
        await expect(page.locator('#widget-preview')).toHaveClass(/dark/);

        // Check soft paywall
        const removeBrandingCheckbox = page.locator('#remove-branding');
        await removeBrandingCheckbox.click();

        // The paywall should appear if we don't have pro
        const paywallModal = page.locator('#paywall-modal');
        await expect(paywallModal).toHaveClass(/active/);

        // Close paywall
        await page.locator('#close-paywall').click();

        // Generate embed code
        const generateBtn = page.locator('#get-code-btn');
        await generateBtn.click();

        // Check the modal
        const embedModal = page.locator('#embed-modal');
        await expect(embedModal).toHaveClass(/active/);

        // Check the code
        const embedCode = await page.locator('#embed-code').inputValue();
        expect(embedCode).toContain('embed/waitlist');
        expect(embedCode).toContain('product=Awesome%20New%20Gadget');
        expect(embedCode).toContain('goal=5');
        expect(embedCode).toContain('theme=dark');
        expect(embedCode).toContain('hideBranding=false');

        // Close modal
        await page.locator('#close-embed').click();
    });
});
