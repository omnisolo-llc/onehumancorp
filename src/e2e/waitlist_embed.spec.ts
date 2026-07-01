import { test, expect } from '@playwright/test';

test.describe('Waitlist Embed Widget', () => {
    test('renders waitlist embed with correct branding and product name', async ({ page }) => {
        // Go to the new embed route we just fixed
        await page.goto('/api/v1/growth/waitlist/embed?tenant=test-tenant&product=Awesome%20Product&goal=5&theme=light&hideBranding=false');

        // Check if the title is correctly set
        await expect(page.locator('h2')).toContainText('Join the Awesome Product Waitlist');

        // Check description text
        await expect(page.locator('p').first()).toContainText('Be the first to access our new launch. Refer 5 friends to jump to the front of the line!');

        // Check for OHC branding
        const brandingLink = page.locator('a:has-text("⚡ Powered by OHC")');
        await expect(brandingLink).toBeVisible();
        await expect(brandingLink).toHaveAttribute('href', /ref=test-tenant/);

        // Check the form and inputs
        await expect(page.locator('input[type="email"]')).toBeVisible();
        await expect(page.locator('button:has-text("Join Waitlist")')).toBeVisible();
    });
});
