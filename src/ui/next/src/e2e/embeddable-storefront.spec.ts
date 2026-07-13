import { test, expect } from '../../../../e2e/fixtures';

test.describe('Embeddable Storefront Widget Growth Loop', () => {
    test('dashboard shows the embed storefront widget and copies HTML', async ({ page }) => {
        // Go to dashboard
        await page.goto('/dashboard');

        // Look for the "Storefront Widget" link in the Dashboard Growth & Virality section
        const widgetLink = page.locator('text=Storefront Widget');
        await expect(widgetLink).toBeVisible();
        await widgetLink.click();

        // Should now be on the Storefront Widget page
        const sectionHeader = page.locator('h1', { hasText: 'Embed Your Store' });
        await expect(sectionHeader).toBeVisible({ timeout: 10000 });

        // Check for the "New Growth Loop" badge next to the header
        await expect(page.locator('text=New Growth Loop').first()).toBeVisible();

        // Click "Get Widget" button
        const getWidgetBtn = page.locator('button:has-text("Get Widget")');
        await expect(getWidgetBtn).toBeVisible();
        await getWidgetBtn.click();

        // Modal should appear
        const modalHeader = page.getByRole('heading', { name: /Embed Storefront/i });
        await expect(modalHeader).toBeVisible();

        // The textarea should contain the iframe snippet
        const textarea = page.locator('textarea').filter({ hasText: '<iframe src="https://ohc.app/api/v1/growth/storefront/embed' });
        await expect(textarea).toBeVisible();

        // Verify the HTML snippet structure (e.g. contains the theme=light, width, height, frameborder, etc.)
        const snippet = await textarea.inputValue();
        expect(snippet).toContain('<iframe src="https://ohc.app/api/v1/growth/storefront/embed?tenant=');
        expect(snippet).toContain('theme=light');
        expect(snippet).toContain('width="320"');
        expect(snippet).toContain('height="400"');
        expect(snippet).toContain('frameborder="0"');

        // Wait a little bit just to make sure things render nicely
        await page.waitForTimeout(500);
    });

    test('embed API endpoint returns the storefront HTML', async ({ request }) => {
        const response = await request.get('/api/v1/growth/storefront/embed?tenant=maya-cakes&theme=light');
        expect(response.ok()).toBeTruthy();

        const html = await response.text();

        // Assert the HTML contains the correct structure and elements
        expect(html).toContain('<!DOCTYPE html>');

        // Use generic testing assertions that work with both the Rust backend and Next.js frontend route implementations
        expect(html).toContain('Buy Now');

        // Ensure the referral growth loop is intact in the footer
        expect(html).toContain('Powered by');
        expect(html).toContain('OHC');
    });
});
