import { test, expect } from './fixtures';

test.describe('Embeddable Storefront Widget Growth Loop', () => {
    test('1. dashboard shows the embed storefront widget and copies HTML', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard');
        await page.waitForLoadState('networkidle');

        // Look for the "Storefront Widget" link in the Dashboard Growth & Virality section
        const widgetLink = page.locator('a[href="storefront-widget.html"]').first();
        await expect(widgetLink).toBeVisible();
        await widgetLink.click();

        await page.waitForLoadState('networkidle');

        // Should now be on the Storefront Widget page
        const sectionHeader = page.locator('h2', { hasText: /Widget Settings/i });
        await expect(sectionHeader).toBeVisible();

        // Click "Get Widget" button
        const getWidgetBtn = page.locator('#get-widget-btn');
        await expect(getWidgetBtn).toBeVisible();
        await getWidgetBtn.click();

        // Modal should appear
        const modalHeader = page.locator('h2:has-text("Embed Storefront")');
        await expect(modalHeader).toBeVisible();

        // The textarea should contain the iframe snippet
        const textarea = page.locator('textarea').first();
        await expect(textarea).toBeVisible();

        // Verify the HTML snippet structure
        const snippet = await textarea.inputValue();
        expect(snippet).toContain('<iframe src="https://ohc.app/api/v1/growth/storefront/embed?tenant=');
        expect(snippet).toContain('theme=light');
        expect(snippet).toContain('width="320"');
        expect(snippet).toContain('height="400"');
        expect(snippet).toContain('frameborder="0"');
    });

    test('2. storefront widget builder updates embed code when theme toggle is clicked', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/storefront-widget.html');
        await page.waitForLoadState('networkidle');

        const darkThemeBtn = page.locator('#theme-dark').first();
        await expect(darkThemeBtn).toBeVisible();
        await darkThemeBtn.click();

        const getWidgetBtn = page.locator('#get-widget-btn');
        await getWidgetBtn.click();

        const textarea = page.locator('textarea').first();
        await expect(textarea).toBeVisible();

        const snippet = await textarea.inputValue();
        expect(snippet).toContain('theme=dark');
    });

    test('3. embed API endpoint returns the storefront HTML for light theme', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        const response = await page.request.get('/api/v1/growth/storefront/embed?tenant=maya-cakes&theme=light');
        expect(response.ok()).toBeTruthy();

        const html = await response.text();
        expect(html).toContain('<!DOCTYPE html>');
        expect(html).toContain('Powered by');
        expect(html).toContain('OHC');
        expect(html).toContain('--background: #ffffff;');
    });

    test('4. embed API endpoint returns the storefront HTML for dark theme', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        const response = await page.request.get('/api/v1/growth/storefront/embed?tenant=maya-cakes&theme=dark');
        expect(response.ok()).toBeTruthy();

        const html = await response.text();
        expect(html).toContain('<!DOCTYPE html>');
        expect(html).toContain('--background: #111111;');
        expect(html).toContain('--text: #ffffff;');
    });

    test('5. embed API endpoint handles missing parameters gracefully', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        const response = await page.request.get('/api/v1/growth/storefront/embed');
        expect(response.ok()).toBeTruthy();

        const html = await response.text();
        expect(html).toContain('<!DOCTYPE html>');
        expect(html).toContain('Signature Watch');
        expect(html).toContain('$299.00');
    });
});
