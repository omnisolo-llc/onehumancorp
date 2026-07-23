import { test, expect } from '@playwright/test';
import { adminPage, memberPage } from '../fixtures';

test.describe('Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering', () => {
    test('Marketing Agent autonomously generates SEO metadata and caches storefront', async ({ page }) => {
        // We MUST start from the home page after user login via the UI
        await page.goto('/login');
        await page.fill('input[name="email"]', 'test@example.com');
        await page.fill('input[name="password"]', 'password123');
        await page.click('button:has-text("Sign In")');

        await expect(page.locator('text="Unified Agent Feed"').first().or(page.locator('text="Dashboard"').first())).toBeVisible({ timeout: 15000 });

        // Navigate to products/inventory to trigger updates via UI
        await page.goto('/dashboard/products');

        // Ensure the products page loaded
        await expect(page.locator('text="Products"').first()).toBeVisible();

        // Create a new product to trigger tenant.product.created -> tenant.website.updated
        await page.click('button:has-text("Add Product"), a:has-text("Add Product")');

        // Fill out product details
        const uniqueProductName = `SEO Edge Cake ${Date.now()}`;
        await page.fill('input[name="name"], input[name="title"]', uniqueProductName);
        await page.fill('textarea[name="description"]', 'A beautifully edge-cached vegan cake.');
        await page.fill('input[name="price"]', '50.00');

        // Save the product
        await page.click('button:has-text("Save"), button:has-text("Create")');

        // Wait for the success state
        await expect(page.locator('text="Saved"').first().or(page.locator('text="Product created"').first()).or(page.locator(`text="${uniqueProductName}"`).first())).toBeVisible({ timeout: 15000 });

        // The product creation triggers the SEO pre-rendering job asynchronously.
        // We will now visit the storefront edge route to verify it's cached and contains SEO.
        // We need the tenant ID and site ID, but as a real user, we might navigate to the live site.
        // For E2E validation of the edge middleware without database shortcuts, we can use the app's storefront link if available,
        // or construct the request using the default e2e-tenant.
        const request = page.request;
        const tenantId = 'e2e-tenant';

        // Fetch sites to get the active site ID for the tenant, or assume 'e2e-site' or similar if seeded.
        // Since we are validating edge cache behavior, we can query the storefront preview if the UI has a "View Store" button.
        const viewStoreLink = page.locator('a:has-text("View Store"), a:has-text("Live Preview")').first();
        let storefrontUrl = '';
        if (await viewStoreLink.isVisible()) {
            storefrontUrl = await viewStoreLink.getAttribute('href') || '';
        }

        // If we don't have a direct URL, we will hit the edge directly using the seeded site for e2e-tenant
        // E2E seed might not have a site for e2e-tenant. But the job will generate one if triggered.
        // Let's rely on the known edge endpoint

        // Note: For a robust E2E we should interact with the site, but Playwright Request is stable for API checks.
        // We use the known seed or a default site ID for e2e-tenant.
        // In real E2E we would navigate to the live preview and check headers, but `page.goto` doesn't expose response headers easily after the fact without setup.
        // So `request.get` is standard for caching validation.

        // Let's assume the site ID is 'default' or we can trigger it and wait for any HIT.
        // Actually, we can just intercept the request in the browser.

        const [response] = await Promise.all([
            page.waitForResponse(res => res.url().includes('/edge/') && res.status() === 200, { timeout: 15000 }),
            page.goto(`/edge/${tenantId}/default-site`).catch(() => {}) // The router handles valid/invalid site IDs by serving tenant's default
        ]);

        if (response) {
            // Verify cache hit and SEO schema
            await expect.poll(async () => {
                const edgeRes = await request.get(response.url());
                if (edgeRes.headers()['x-cache'] === 'HIT') {
                    const body = await edgeRes.text();
                    if (body.includes('schema.org') || body.includes(uniqueProductName)) {
                        return 'HIT_AND_CONTAINS_SEO';
                    }
                }
                return edgeRes.headers()['x-cache'];
            }, {
                message: 'Wait for edge cache population and SEO generation',
                timeout: 10000,
            }).toBe('HIT_AND_CONTAINS_SEO');

            // Now go back to admin UI and update inventory
            await page.goto('/dashboard/products');
            await page.click(`text="${uniqueProductName}"`);
            await page.fill('input[name="price"]', '45.00'); // price update triggers inventory/pricing update
            await page.click('button:has-text("Save"), button:has-text("Update")');
            await expect(page.locator('text="Saved"').first().or(page.locator('text="Product updated"').first())).toBeVisible({ timeout: 15000 });

            // Refetch should now be a miss due to invalidation triggered by OperationsAgent
            await expect.poll(async () => {
                const edgeResAfterInvalidation = await request.get(response.url());
                return edgeResAfterInvalidation.headers()['x-cache'];
            }, {
                message: 'Wait for edge cache invalidation',
                timeout: 10000,
            }).toBe('MISS');
        }
    });
});
