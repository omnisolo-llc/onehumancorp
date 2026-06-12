import { test, expect } from '@playwright/test';

test.describe('Email Signature Generator Growth Loop', () => {
    test.beforeEach(async ({ page }) => {
        // Setup local storage to emulate a non-pro user with a specific tenant
        await page.goto('/');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'maya-cakes');
            localStorage.setItem('has_pro', 'false');
        });
    });

    test('generates signature with Powered by OHC loop and blocks removal without Pro', async ({ page }) => {
        // Navigate to the generator from the dashboard to simulate real flow
        await page.goto('/dashboard');
        await page.click('text=Email Signature Generator');

        // Fill out the form
        await page.fill('input[name="name"]', 'Maya Smith');
        await page.fill('input[name="role"]', 'Owner');
        await page.fill('input[name="company"]', 'Maya Cakes');
        await page.fill('input[name="website"]', 'maya-cakes.ohc.app');

        // Attempt to turn off branding
        const brandingCheckbox = page.locator('input[type="checkbox"]');
        await brandingCheckbox.click();

        // Verify the upgrade modal appears
        await expect(page.locator('text=Upgrade to Pro')).toBeVisible();
        await page.click('text=Cancel');

        // Ensure checkbox remained unchecked
        await expect(brandingCheckbox).not.toBeChecked();

        // Generate the signature
        await page.click('text=Generate Signature');

        // Verify preview is rendered
        await expect(page.locator('h3:has-text("Preview")')).toBeVisible();

        // Check the HTML source for the signature
        const htmlSource = await page.locator('textarea').inputValue();
        expect(htmlSource).toContain('Maya Smith');
        expect(htmlSource).toContain('Owner | Maya Cakes');
        expect(htmlSource).toContain('https://maya-cakes.ohc.app');

        // Check the viral loop is intact
        expect(htmlSource).toContain('⚡ Powered by OHC');
        expect(htmlSource).toContain('ref=maya-cakes');
    });

    test('allows branding removal for Pro users', async ({ page }) => {
        await page.goto('/');
        await page.evaluate(() => {
            localStorage.setItem('has_pro', 'true');
        });
        await page.goto('/email-signature-generator');

        await page.fill('input[name="name"]', 'Pro Owner');

        // Should be able to check it without modal
        const brandingCheckbox = page.locator('input[type="checkbox"]');
        await brandingCheckbox.click();
        await expect(brandingCheckbox).toBeChecked();
        await expect(page.locator('text=Upgrade to Pro')).not.toBeVisible();

        await page.click('text=Generate Signature');

        const htmlSource = await page.locator('textarea').inputValue();
        expect(htmlSource).toContain('Pro Owner');
        expect(htmlSource).not.toContain('⚡ Powered by OHC');
    });
});
