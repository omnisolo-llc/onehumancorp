import { test, expect } from '@playwright/test';

test.describe('Testimonial Widget Generator E2E', () => {
    test('User can configure testimonial and copy embed code', async ({ page }) => {
        await page.goto('/testimonial-widget');

        await expect(page.getByRole('heading', { name: 'Testimonial Widget 🌟' })).toBeVisible();

        await page.fill('input[placeholder="e.g. my-store"]', 'awesome-bakery');
        await page.fill('input[placeholder="e.g. Jane Doe"]', 'Maya The Baker');
        await page.fill('textarea', 'The cakes are absolutely amazing!');
        await page.selectOption('select', '4');

        await page.getByRole('button', { name: 'Dark' }).click();

        // Check if viral loop option is present and showing PRO badge
        await expect(page.getByText('Remove "Powered by OHC" Badge')).toBeVisible();

        // Verify soft paywall appears when checking without Pro
        const removeBrandingCheckbox = page.getByLabel('Remove "Powered by OHC" Badge');
        await removeBrandingCheckbox.check();

        const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Remove Branding' });
        await expect(paywallHeading).toBeVisible();
        await expect(page.getByText('Make the Testimonial Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.')).toBeVisible();

        // Close paywall
        await page.getByRole('button', { name: 'Close paywall' }).click();

        // The checkbox should be unchecked since we don't have Pro
        await expect(removeBrandingCheckbox).not.toBeChecked();

        await page.getByRole('button', { name: 'Get Widget Code' }).click();

        const modalHeading = page.getByRole('heading', { name: 'Embed Testimonial' });
        await expect(modalHeading).toBeVisible();

        const embedTextarea = page.locator('textarea[readonly]');
        const embedValue = await embedTextarea.inputValue();
        expect(embedValue).toContain('<iframe');
        expect(embedValue).toContain('api/v1/growth/testimonial/embed');
        expect(embedValue).toContain('tenant=awesome-bakery');
        expect(embedValue).toContain('authorName=Maya%20The%20Baker');
        expect(embedValue).toContain('theme=dark');
        expect(embedValue).toContain('branding=true');

        await page.getByRole('button', { name: 'Copy Code' }).click();

        await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();

        await page.getByRole('button', { name: 'Close' }).click();
        await expect(modalHeading).not.toBeVisible();
    });
});