import { test, expect } from './fixtures';

test.describe('Documentation Features Flow', () => {

    test('Help Center page UI loads and structure is visible', async ({ page }) => {
        // Go to the help widget UI
        await page.goto('/help.html');
        await expect(page.locator('h1').filter({ hasText: 'In-App Help Center' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'Getting Started' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'My Store' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'Payments' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'AI Agents' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'Marketing' })).toBeVisible();
        await expect(page.locator('h3').filter({ hasText: 'Account & Billing' })).toBeVisible();
    });

    test('Changelog UI loads', async ({ page }) => {
        await page.goto('/changelog.html');
        await expect(page.locator('h1').filter({ hasText: 'Release Notes & Changelog' })).toBeVisible();
    });

    test('API Docs alias loads the maintained reference and survives reload', async ({ page }) => {
        const loaded = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/api-docs-spec' && response.request().method() === 'GET');
        await page.goto('/api-docs.html?source=scribe&tag=one&tag=two#reference');
        await expect(page).toHaveURL(url => url.pathname === '/api-docs'
            && url.hash === '#reference'
            && JSON.stringify([...url.searchParams]) === JSON.stringify([['source', 'scribe'], ['tag', 'one'], ['tag', 'two']]));
        const response = await loaded;
        expect(response.status()).toBe(200);
        const spec = await response.json();
        expect(spec.info.title).toBe('API Documentation (for Advanced Users)');
        await expect(page.getByTestId('api-docs-title')).toContainText('Advanced:');
        await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui .info .title')).toBeVisible();
        await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui .info .title')).toContainText(spec.info.title);

        const reloaded = page.waitForResponse(response => new URL(response.url()).pathname === '/api/v1/api-docs-spec' && response.request().method() === 'GET');
        await page.reload();
        const refresh = await reloaded;
        expect(refresh.status()).toBe(200);
        expect(await refresh.json()).toEqual(spec);
        await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui .info .title')).toBeVisible();
        await expect(page.frameLocator('iframe[data-ohc-api-docs-viewer]').locator('.swagger-ui .info .title')).toContainText(spec.info.title);
    });

});
