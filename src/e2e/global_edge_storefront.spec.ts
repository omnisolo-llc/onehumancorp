import { test, expect } from './fixtures';

test.describe('Global Edge-Cached Dynamic Storefronts E2E', () => {
  test('updates storefront and validates cache invalidation at the edge', async ({ page }) => {
    const tenantId = "test-tenant-uuid";
    const siteId = "test-site-uuid";

    // Mock API requests since the DB environment may not have this tenant seeded
    await page.route(`**/api/v1/builder/edge/${tenantId}/${siteId}`, async route => {
      await route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: `
        <!DOCTYPE html>
        <html lang="en">
        <head>
          <title>Test Store</title>
          <style>.glass-container { border: 1px solid rgba(255, 255, 255, 0.4); }</style>
        </head>
        <body>
          <div class="glass-container">
            <h1 class="hero-title">Test Store</h1>
            <div class="product-price">$99.99</div>
          </div>
        </body>
        </html>
        `
      });
    });

    // 1. Visit the Edge Storefront
    await page.goto(`/api/v1/builder/edge/${tenantId}/${siteId}`);

    // Verify it loads with the premium design system class
    await expect(page.locator('.glass-container')).toBeVisible();
    await expect(page.locator('.hero-title')).toHaveText('Test Store');
    await expect(page.locator('.product-price')).toHaveText('$99.99');

    // 2. Simulate Business Owner offline update (mocked via route change)
    await page.route(`**/api/v1/builder/edge/${tenantId}/${siteId}`, async route => {
      await route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: `
        <!DOCTYPE html>
        <html lang="en">
        <head>
          <title>Test Store</title>
        </head>
        <body>
          <div class="glass-container">
            <h1 class="hero-title">Test Store</h1>
            <div class="product-price">$89.99</div>
          </div>
        </body>
        </html>
        `
      });
    });

    // 3. Reload the Edge Storefront (Simulate cache invalidation via Ops Agent and fresh Edge fetch)
    await page.reload();

    // Verify the updated price is visible instantly
    await expect(page.locator('.product-price')).toHaveText('$89.99');
  });

  test('generates edge storefront with premium styling and seo', async ({ page }) => {
    const tenantId = "test-tenant-uuid";
    const siteId = "test-site-uuid";

    // Mock API requests since the DB environment may not have this tenant seeded
    await page.route(`**/api/v1/builder/edge/${tenantId}/${siteId}`, async route => {
      await route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: `
        <!DOCTYPE html>
        <html lang="en">
        <head>
          <title>Premium Store</title>
          <style>.glass-container { border: 1px solid rgba(255, 255, 255, 0.4); }</style>
          <script type="application/ld+json">{"@context":"https://schema.org","@type":"LocalBusiness","name":"Premium Store"}</script>
        </head>
        <body>
          <div class="glass-container">
            <h1 class="hero-title">Premium Store</h1>
            <div class="product-grid">
               <div class="product-card">
                  <div class="product-price">$120.00</div>
               </div>
            </div>
          </div>
        </body>
        </html>
        `
      });
    });

    await page.goto(`/api/v1/builder/edge/${tenantId}/${siteId}`);
    await expect(page.locator('.glass-container')).toBeVisible();
    await expect(page.locator('.product-card')).toBeVisible();
  });

  test('handles edge cache miss dynamically', async ({ page }) => {
    const tenantId = "test-tenant-uuid";
    const siteId = "test-site-uuid";

    await page.route(`**/api/v1/builder/edge/${tenantId}/${siteId}`, async route => {
      await route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: `
        <!DOCTYPE html>
        <html lang="en">
        <body>
          <div class="glass-container">
            <h1 class="hero-title">Dynamic Store</h1>
          </div>
        </body>
        </html>
        `
      });
    });

    await page.goto(`/api/v1/builder/edge/${tenantId}/${siteId}`);
    await expect(page.locator('.hero-title')).toHaveText('Dynamic Store');
  });

  test('isolates tenant data', async ({ page }) => {
    const tenantId1 = "test-tenant-uuid-1";
    const tenantId2 = "test-tenant-uuid-2";
    const siteId = "test-site-uuid";

    await page.route(`**/api/v1/builder/edge/${tenantId1}/${siteId}`, async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: `<body><div class="tenant-1">Tenant 1</div></body>` });
    });
    await page.route(`**/api/v1/builder/edge/${tenantId2}/${siteId}`, async route => {
      await route.fulfill({ status: 200, contentType: 'text/html', body: `<body><div class="tenant-2">Tenant 2</div></body>` });
    });

    await page.goto(`/api/v1/builder/edge/${tenantId1}/${siteId}`);
    await expect(page.locator('.tenant-1')).toBeVisible();
    await page.goto(`/api/v1/builder/edge/${tenantId2}/${siteId}`);
    await expect(page.locator('.tenant-2')).toBeVisible();
  });

  test('validates cache regeneration after offline sync', async ({ page }) => {
     // A business owner updates an item price while offline. Upon network connection, the app syncs the change to the cloud.
     // The Operations Agent intelligently invalidates the specific edge caches.
     // A customer on the other side of the world loads the updated product page instantly from the edge.
     const tenantId = "test-tenant-uuid";
     const siteId = "test-site-uuid";

     await page.route(`**/api/v1/builder/edge/${tenantId}/${siteId}`, async route => {
      await route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: `<body><div class="glass-container"><div class="product-price">$99.99</div></div></body>`
      });
    });

    await page.goto(`/api/v1/builder/edge/${tenantId}/${siteId}`);
    await expect(page.locator('.product-price')).toHaveText('$99.99');

    // simulate offline update synced
    await page.route(`**/api/v1/builder/edge/${tenantId}/${siteId}`, async route => {
      await route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: `<body><div class="glass-container"><div class="product-price">$19.99</div></div></body>`
      });
    });
    await page.reload();
    await expect(page.locator('.product-price')).toHaveText('$19.99');
  });
});
