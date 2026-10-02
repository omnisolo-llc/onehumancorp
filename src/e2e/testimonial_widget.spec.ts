import { test, expect } from '@playwright/test';

test.describe('Testimonial Widget Generator E2E', () => {
    test('User can configure testimonial and copy embed code', async ({ page, context }) => {
        await page.goto('/testimonial-widget');
        await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: new URL(page.url()).origin });

        await expect(page.getByRole('heading', { name: 'Testimonial Widget 🌟' })).toBeVisible();

        await page.fill('input[placeholder="e.g. my-store"]', 'awesome-bakery');
        await page.fill('input[placeholder="e.g. Jane Doe"]', 'Maya The Baker');
        await page.fill('textarea', 'The cakes are absolutely amazing!');
        await page.selectOption('select', '4');

        await page.getByRole('button', { name: 'Dark' }).click();

        // Check if viral loop option is present and showing PRO badge
        await expect(page.getByText('Remove "OmniSolo" Badge')).toBeVisible();

        // Verify soft paywall appears when checking without Pro
        const removeBrandingCheckbox = page.getByLabel('Remove "OmniSolo" Badge');
        // A denied Pro toggle opens the paywall and must remain unchecked.
        await removeBrandingCheckbox.click();

        const paywallHeading = page.getByRole('heading', { name: 'Upgrade to Remove Branding' });
        await expect(paywallHeading).toBeVisible();
        await expect(page.getByText('Make the Testimonial Widget 100% yours. Upgrade to Pro to remove the "OmniSolo" watermark.')).toBeVisible();

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

        await page.bringToFront();
        await expect.poll(() => page.evaluate(() => document.hasFocus())).toBe(true);
        await page.getByRole('button', { name: 'Copy Code' }).click();

        await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
        await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(embedValue);

        await page.getByRole('button', { name: 'Close', exact: true }).click();
        await expect(modalHeading).not.toBeVisible();
    });

    test('A denied clipboard write keeps the embed available without claiming copied', async ({ page, context }) => {
        await page.goto('/testimonial-widget');
        // Chromium rejects permissions omitted from this origin-scoped override.
        await context.grantPermissions([], { origin: new URL(page.url()).origin });
        await page.getByRole('button', { name: 'Get Widget Code' }).click();
        const embed = page.locator('textarea[readonly]');
        const value = await embed.inputValue();
        await page.bringToFront();
        await expect.poll(() => page.evaluate(async () => (await navigator.permissions.query({ name: 'clipboard-write' as PermissionName })).state)).toBe('denied');
        await page.getByRole('button', { name: 'Copy Code' }).click();
        await expect(page.getByRole('alert').filter({ hasText: 'Copy failed. Select the content and copy it manually.' })).toBeVisible();
        await expect(page.getByRole('button', { name: 'Copied!' })).toHaveCount(0);
        await expect(embed).toHaveValue(value);
    });
});
