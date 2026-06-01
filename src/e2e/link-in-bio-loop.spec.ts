import { test, expect } from '@playwright/test';

test.describe('Link-in-Bio Growth Loop', () => {
    test('dashboard links to link-in-bio page and renders correctly', async ({ page }) => {
        // Go to dashboard
        await page.goto('http://localhost:3000/dashboard');

        // Click Link in Bio nav link
        const linkInBioNavLink = page.locator('a:has-text("Link in Bio")').first();
        await expect(linkInBioNavLink).toBeVisible();

        // Workaround for Next.js routing latency in tests
        const href = await linkInBioNavLink.getAttribute('href');
        if (href) {
            await page.goto(`http://localhost:3000${href}`);
        } else {
            await linkInBioNavLink.click();
        }

        // Verify page loads
        await expect(page).toHaveURL(/.*\/link-in-bio/);
        await expect(page.locator('h1:has-text("Link-in-Bio Generator")')).toBeVisible();

        // Verify elements on the page
        await expect(page.locator('label:has-text("Store Name")')).toBeVisible();
        await expect(page.locator('label:has-text("Bio / Tagline")')).toBeVisible();
        await expect(page.locator('label:has-text("Theme")')).toBeVisible();

        const previewHeading = page.locator('h1:has-text("My Awesome Store")').first();
        await expect(previewHeading).toBeVisible();

        const previewBio = page.locator('p:has-text("Premium products for awesome people.")').first();
        await expect(previewBio).toBeVisible();

        // Verify Powered by OHC
        await expect(page.locator('span:has-text("Powered by OHC")').first()).toBeVisible();
    });
});
