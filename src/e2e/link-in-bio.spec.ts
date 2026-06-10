import { test, expect } from '@playwright/test';

test.describe('Link-in-Bio Generator Growth Loop', () => {
    test('generator page renders correctly, saves data, and public page works with footer', async ({ page }) => {
        // 1. Set some initial local storage state to act as a logged-in user
        await page.goto('/dashboard');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-bakery');
        });

        // 2. Go to the Link-in-Bio Generator page
        await page.goto('/link-in-bio-generator');

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Link-in-Bio Generator' })).toBeVisible();

        // 3. Configure the bio page
        const businessNameInputs = page.locator('input');
        // The first input is usually the store name based on layout
        await businessNameInputs.first().fill('Awesome E2E Bakery');

        const bioTextarea = page.locator('textarea');
        await bioTextarea.fill('The best automated cakes in town.');

        // 4. Verify preview updates in real-time
        await expect(page.locator('h1', { hasText: 'Awesome E2E Bakery' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'The best automated cakes in town.' })).toBeVisible();

        // Check the "Powered by OHC" footer in the live preview
        const previewFooterLink = page.locator('a', { hasText: 'Powered by OHC' });
        await expect(previewFooterLink).toBeVisible();
        await expect(previewFooterLink).toHaveAttribute('href', /^https:\/\/ohc\.store\/join\?ref=e2e-bakery/);

        // Wait a moment for the useEffect to save to localStorage
        await page.waitForTimeout(500);

        // 5. Navigate to the generated public page
        await page.goto('/bio/e2e-bakery');

        // Verify the public page loaded the saved data
        await expect(page.locator('h1', { hasText: 'Awesome E2E Bakery' })).toBeVisible();
        await expect(page.locator('p', { hasText: 'The best automated cakes in town.' })).toBeVisible();

        // Verify the viral footer exists on the public page
        const publicFooterLink = page.locator('a', { hasText: 'Powered by OHC' });
        await expect(publicFooterLink).toBeVisible();
        await expect(publicFooterLink).toHaveAttribute('href', 'https://ohc.store/join?ref=e2e-bakery');
    });

    test('Dashboard contains link to Link-in-Bio generator', async ({ page }) => {
        await page.goto('/dashboard');

        // Find the link to create a link in bio page
        const linkInBioButton = page.locator('a[href="/link-in-bio-generator"]');
        await expect(linkInBioButton).toBeVisible();
        await expect(linkInBioButton).toContainText('Create Link-in-Bio Page');
    });
});
