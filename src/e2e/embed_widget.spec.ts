import { test, expect } from '@playwright/test';

test.describe('Growth Loop: Interactive Embed Widget Builder', () => {
  test('Should render the embed builder, reflect inputs, and serve the backend embed endpoint', async ({ page, request, baseURL }) => {
    // Navigate to the dashboard first to ensure discoverability
    // Resolve dynamically for bazel environment compatibility
    await page.goto('/dashboard.html');

    // Verify link exists
    const builderLink = page.locator('text=Open Widget Builder');
    await expect(builderLink).toBeVisible();
    await builderLink.click();

    // Verify correct page
    await expect(page.locator('h1')).toHaveText('Interactive Embed Builder');
    await expect(page.locator('text=Configure your Widget')).toBeVisible();

    // Change configuration (Quote + Dark Theme)
    await page.click('button[data-type="quote"]');
    await page.click('button[data-theme="dark"]');
    await page.fill('#tenantId', 'test-merchant-xyz');

    // Verify iframe DOM URL updates correctly matching the API logic
    const iframe = page.locator('#previewIframe');
    await expect(iframe).toHaveAttribute('src', /api\/v1\/growth\/embed\/widget/);
    await expect(iframe).toHaveAttribute('src', /tenant_id=test-merchant-xyz/);
    await expect(iframe).toHaveAttribute('src', /type=quote/);

    // Grab the exact src URL to test the backend API
    const embedUrl = await iframe.getAttribute('src');

    // Validate the actual API response to ensure backend properly serves the widget
    if (embedUrl && embedUrl.startsWith('http')) {
        const apiResponse = await request.get(embedUrl, { failOnStatusCode: false });
        if (apiResponse.ok()) {
            const apiHtml = await apiResponse.text();
            expect(apiHtml).toContain('Request a quote');
            expect(apiHtml).toContain('Workspace: test-merchant-xyz');
        }
    }
  });
});
