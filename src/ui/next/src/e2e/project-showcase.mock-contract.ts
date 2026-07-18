import { test, expect } from '../../../../e2e/fixtures';

test.describe('Project Showcase Generator (Growth Loop)', () => {
    test.beforeEach(async ({ page }) => {
        // Clear local storage and set tenant so we get a consistent PoweredByOHC link
        await page.goto('/dashboard');
        await page.evaluate(() => {
            window.localStorage.clear();
            window.localStorage.setItem('has_onboarded', 'true');
            window.localStorage.setItem('tenant', 'demo-tenant');
        });
    });

    test('renders form, updates preview, and displays "Powered by OHC"', async ({ page }) => {
        // Go to the Project Showcase generator page
        await page.goto('/project-showcase');

        // Fill out the project details
        await page.fill('input[placeholder="e.g. Modern Kitchen Remodel"]', 'A Beautiful New Kitchen');
        await page.fill('input[placeholder="e.g. The Smith Family"]', 'The Smiths');
        await page.fill('textarea[placeholder="Describe the work done, the challenges, and the outcome..."]', 'Replaced all the cabinets and installed new granite countertops.');

        // Ensure preview updates
        await expect(page.locator('h1', { hasText: 'A Beautiful New Kitchen' }).first()).toBeVisible();
        await expect(page.locator('p', { hasText: 'For The Smiths' }).first()).toBeVisible();

        // 1. Verify "Powered by OHC" watermark is visible in the preview area
        const watermark = page.locator('a', { hasText: /Powered by OHC/i }).first();
        await expect(watermark).toBeVisible();

        // The link should direct back to OHC with the tenant as a reference source
        await expect(watermark).toHaveAttribute('href', /.*\/onboarding\?ref=demo-tenant.*/);

        // 2. Click the "Remove Branding" toggle (simulating free user)
        const toggle = page.locator('input[type="checkbox"]');
        await toggle.evaluate((el: HTMLInputElement) => el.click()); // force click on hidden input

        // 3. Verify that the Pro Paywall modal appears instead of removing the branding
        const paywallModal = page.locator('div:has-text("Upgrade to Pro")').last();
        await expect(paywallModal).toBeVisible();
        await expect(page.locator('h2', { hasText: 'Upgrade to Pro' })).toBeVisible();

        // Close the modal
        await page.locator('button', { hasText: 'Maybe Later' }).click();
        await expect(page.locator('h2', { hasText: 'Upgrade to Pro' })).not.toBeVisible();

        // 4. Test sharing link creation functionality
        const copyButton = page.locator('button', { hasText: 'Copy Share Link' });
        await copyButton.click();

        // Should show copied status
        await expect(page.locator('button', { hasText: '✓ Link Copied!' })).toBeVisible();

        // 5. Verify the actual generated page end-to-end
        const params = new URLSearchParams({
            p: 'A Beautiful New Kitchen',
            c: 'The Smiths',
            d: 'Replaced all the cabinets and installed new granite countertops.',
            r: '0',
            t: 'demo-tenant'
        });

        await page.goto(`/showcase?${params.toString()}`);
        await expect(page.locator('h1', { hasText: 'A Beautiful New Kitchen' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'For The Smiths' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'Replaced all the cabinets and installed new granite countertops.' })).toBeVisible();

        // The public showcase should also have the powered by watermark
        const publicWatermark = page.locator('a', { hasText: /Powered by OHC/i }).first();
        await expect(publicWatermark).toBeVisible();
    });

    test('allows pro users to toggle branding off', async ({ page }) => {
        // Simulate a Pro user
        await page.goto('/dashboard');
        await page.evaluate(() => {
            window.localStorage.setItem('has_pro', 'true');
        });

        // Go to the Project Showcase generator page
        await page.goto('/project-showcase');

        // Verify watermark is hidden by default for pro users (our component logic sets it to true if pro)
        await expect(page.locator('a', { hasText: /Powered by OHC/i }).first()).not.toBeVisible();

        // Verify the checkbox is checked
        const toggle = page.locator('input[type="checkbox"]');
        await expect(toggle).toBeChecked();

        // Toggle branding back on
        await toggle.evaluate((el: HTMLInputElement) => el.click());

        // Watermark should reappear
        const watermark = page.locator('a', { hasText: /Powered by OHC/i }).first();
        await expect(watermark).toBeVisible();
        await expect(toggle).not.toBeChecked();
    });
});
