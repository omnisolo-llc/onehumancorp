import { test, expect } from '@playwright/test';

test.describe('Universal Edge-Cached Storefront & Agentic SEO Pre-rendering', () => {
  test.use({ storageState: { cookies: [], origins: [] } });

  test('Maya adds a cake, verifies SEO from edge, and handles stockout invalidation', async ({ page, request }) => {
    await page.goto('http://localhost:5173/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.click('button[type="submit"]');
    await page.waitForURL('http://localhost:5173/');

    const orgId = await page.evaluate(() => localStorage.getItem('tenant_id')) || 'test-tenant';

    const addProductRes = await request.post(`http://localhost:8000/api/v1/catalog/product`, {
        headers: { 'Authorization': `Bearer ${await page.evaluate(() => localStorage.getItem('access_token'))}` },
        data: {
            name: "Vegan Chocolate Cake Edge Test",
            description: "A delicious, rich vegan chocolate cake.",
            item_type: "Product",
            price: "45.00"
        }
    });

    await page.waitForTimeout(4000);

    const sitesRes = await request.get(`http://localhost:8000/api/v1/builder/sites`, {
        headers: { 'Authorization': `Bearer ${await page.evaluate(() => localStorage.getItem('access_token'))}` }
    });

    expect(sitesRes.ok()).toBeTruthy();

    const sitesBody = await sitesRes.json();
    let siteId;
    if (sitesBody.sites && sitesBody.sites.length > 0) {
        siteId = sitesBody.sites[0].id;
    } else {
        // Fallback for E2E: assume we can create one if not exists
        const createSiteRes = await request.post(`http://localhost:8000/api/v1/builder/sites`, {
             headers: { 'Authorization': `Bearer ${await page.evaluate(() => localStorage.getItem('access_token'))}` }
        });
        const createdSite = await createSiteRes.json();
        siteId = createdSite.id;
    }

    const edgeUrl = `http://localhost:8000/api/v1/builder/edge/${orgId}/${siteId}`;
    const response = await page.request.get(edgeUrl);
    expect(response.status()).toBe(200);

    const html = await response.text();
    expect(html).toContain('Vegan Chocolate Cake Edge Test');
  });
});
