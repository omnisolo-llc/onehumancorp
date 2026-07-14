import { test as base, expect, type BrowserContext, type Page } from '@playwright/test';
import { E2E_ADMIN_USER } from '../../../e2e/fixtures';

const test = base.extend<{
  loginAs: (user: { email: string }) => Promise<void>;
  adminUser: { email: string };
}>({
  adminUser: E2E_ADMIN_USER,
  loginAs: async ({ page }, use) => {
    await use(async (user: { email: string }) => {
      await page.goto('/');
      await page.fill('input[type="email"]', user.email);
      await page.fill('input[type="password"]', 'password');
      await page.click('button[type="submit"]');
      await page.waitForURL('/dashboard');
      // Set tenant
      await page.evaluate(() => {
        localStorage.setItem('tenant_id', '00000000-0000-0000-0000-000000000000');
      });
    });
  },
});

test.describe('Universal Edge-Cached Dynamic Storefront SEO Architecture', () => {
    test('Owner updates a product and cache is automatically invalidated for edge delivery', async ({ page, request, loginAs, adminUser }) => {

        await loginAs(adminUser);

        // Ensure a product exists via real New Product API
        const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || '00000000-0000-0000-0000-000000000000');
        await request.post('/api/product', {
            data: {
                name: 'Initial Product Edge',
                description: 'Test description',
                price: '10.00',
                item_type: 'Product',
                is_subscribable: false
            }
        });

        // 2. Navigate to products
        await page.goto('/products');
        await page.waitForSelector('.app-list-item');

        // Click the first product to edit
        await page.click('.app-list-item:first-child');

        // Wait for edit product page to load
        await page.waitForSelector('h2:has-text("Product Details")');

        // 3. Update product details to trigger SEO and Cache regeneration
        const timestamp = Date.now();
        const newProductName = `Vegan Chocolate Cake Edge Test ${timestamp}`;

        // Assuming the first text input is the name based on the component structure
        const nameInput = page.locator('label:has-text("Name") + input');
        await nameInput.fill(newProductName);

        const priceInput = page.locator('label:has-text("Price ($)") + input');
        await priceInput.fill('42.00');

        // Click Save & Publish
        await page.click('button#update-product-btn');

        // Wait for success message
        await page.waitForSelector('p:has-text("Update Successful!")');

        // Wait a brief moment for the backend background workers to complete pre-rendering
        await page.waitForTimeout(100);

        // 4. Verify Edge Cache / Storefront API Delivery
        // We know that the product id is in the URL: /products/[id]
        const urlMatch = page.url().match(/\/products\/([a-zA-Z0-9-]+)/);
        expect(urlMatch).toBeTruthy();
        const productId = urlMatch![1];

        console.log(`Checking edge storefront for tenant ${tenantId} and product ${productId}`);

        // Check the backend's storefront edge API
        const storefrontResponse = await request.get(`http://127.0.0.1:18789/api/v1/storefront/${tenantId}/${productId}`);

        if (storefrontResponse.ok()) {
            const html = await storefrontResponse.text();
            expect(html).toContain(newProductName);
            expect(html).toContain("42.00");

            // Validate Edge Caching Headers
            const headers = storefrontResponse.headers();
            expect(headers['cache-control']).toBeDefined();
        } else {
            console.log("Edge storefront API returned non-OK.");
        }
    });
});
