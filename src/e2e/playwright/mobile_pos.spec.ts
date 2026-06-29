import { test, expect } from '@playwright/test';

test.describe('Universal Mobile POS & Tap-to-Pay with Agentic Inventory Sync', () => {
    test.use({ viewport: { width: 375, height: 667 } }); // Mobile viewport

    test('should allow owner to tap-to-pay and see low stock alert', async ({ page, request }) => {
        const tenantId = 'pos_test_tenant_' + Date.now();
        const customerPhone = '+15555551234';

        // 1. Seed the database with a user, tenant, and product
        await request.post('/api/v1/builder/seeder/exec', {
          data: {
            sql: `
              INSERT INTO users (id, email, full_name, is_superadmin)
              VALUES ('pos_user_id', 'pos_user@example.com', 'POS User', false)
              ON CONFLICT DO NOTHING;

              INSERT INTO tenants (id, name, owner_email)
              VALUES ('${tenantId}', 'POS Store', 'pos_user@example.com')
              ON CONFLICT DO NOTHING;

              INSERT INTO products (id, tenant_id, title, description, price_cents, inventory_count, available_quantity)
              VALUES ('pos_prod_1', '${tenantId}', 'Signature Handbag', 'Premium leather handbag', 5000, 6, 6)
              ON CONFLICT DO NOTHING;
            `
          }
        });

        // 2. Login to the application
        await page.goto(`/login?test_email=pos_user@example.com`);
        await page.evaluate((t) => localStorage.setItem('tenant', t), tenantId);
        await page.goto('/dashboard');

        // 3. Verify the Sell In Person link is visible
        const sellInPersonLink = page.locator('h3', { hasText: 'Sell In Person' }).locator('..');
        await expect(sellInPersonLink).toBeVisible({ timeout: 10000 });

        // Verify mobile constraints
        const bodyBox = await page.locator('body').boundingBox();
        expect(bodyBox?.width).toBeLessThanOrEqual(375);

        // 4. Navigate to Sell In Person
        await sellInPersonLink.click();

        // Wait for product catalog to load
        await expect(page.locator('text=Signature Handbag')).toBeVisible({ timeout: 10000 });

        // Add to cart
        await page.locator('text=Signature Handbag').click();

        // Verify cart logic
        const chargeBtn = page.locator('button', { hasText: 'Charge $' });
        await expect(chargeBtn).toBeVisible();
        await chargeBtn.click();

        // In the drawer, tap the tap-to-pay button
        const tapToPayBtn = page.locator('button#tap-to-pay-btn');
        await expect(tapToPayBtn).toBeVisible();

        // Before clicking, ensure it is enabled
        await expect(tapToPayBtn).toBeEnabled();
        await tapToPayBtn.click();

        // Wait for successful transaction
        await expect(page.locator('text=Payment Successful!')).toBeVisible({ timeout: 20000 });

        // 5. Navigate back to dashboard to see low stock alert
        await page.goto('/dashboard');

        // Verify that Operations Agent feed shows a low-stock alert
        const feedSection = page.locator('section', { hasText: 'Unified Agent Feed' }).first();
        await expect(feedSection).toBeVisible({ timeout: 15000 });

        const stockAlert = page.locator('text=Stock for Signature Handbag has dropped to 5.');
        await expect(stockAlert).toBeVisible({ timeout: 15000 });
    });
});
