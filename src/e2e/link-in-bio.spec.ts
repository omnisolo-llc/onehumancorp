import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Link-in-Bio Generator E2E', () => {
    test('should allow member to customize and save link-in-bio', async ({ memberPage }) => {
        // Navigate to Dashboard
        await memberPage.goto('/ui/dashboard.html');

        // Click Link-in-Bio Generator
        await memberPage.click('#link-in-bio-link');

        // Wait for Link-in-Bio page
        await expect(memberPage).toHaveURL(/.*link-in-bio-generator.html/);

        // Edit store name
        await memberPage.fill('#store-name', 'My Awesome Bakery');

        // Edit bio
        await memberPage.fill('#bio-text', 'The best cookies in town.');

        // Select 'dark' theme
        await memberPage.click('.theme-btn[data-theme="dark"]');

        // Verify preview updates
        await expect(memberPage.locator('#preview-title')).toHaveText('My Awesome Bakery');
        await expect(memberPage.locator('#preview-bio')).toHaveText('The best cookies in town.');

        // Copy Link
        await memberPage.click('#copy-btn');
        await expect(memberPage.locator('#copy-btn')).toHaveText('Copied Link!');

        // Wait a little for saveState to flush (it has a 500ms debounce)
        await memberPage.waitForTimeout(1000);

        // Now load the bio.html directly to see if the API actually saved it
        // and if it loads successfully from the backend
        const tenantId = 'e2e-tenant'; // In our fixture
        await memberPage.goto(`/ui/bio.html?tenant=${tenantId}`);

        // Wait for it to fetch
        await memberPage.waitForTimeout(1000);

        // Verify it loaded
        await expect(memberPage.locator('#title')).toHaveText('My Awesome Bakery');
        await expect(memberPage.locator('#bio')).toHaveText('The best cookies in town.');

        // Verify Powered by link
        const poweredBy = memberPage.locator('#powered-by-link');
        await expect(poweredBy).toBeVisible();
        await expect(poweredBy).toContainText('Powered by OHC');
        await expect(poweredBy).toHaveAttribute('href', `https://ohc.store/join?ref=${tenantId}`);
    });
});
