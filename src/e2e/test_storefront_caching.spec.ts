import { test, expect } from './fixtures';

test.describe('Edge Caching Dynamic Storefronts', () => {
  test('Instant cache invalidation on inventory update', async ({ page, request }) => {
    // 1. Log in / get session context for the owner (simulate Maya)
    // Setup initial product state via API
    const authHeaders = {
        'x-spiffe-id': 'spiffe://trust_domain/ns/default/org/maya_bakery/agent/owner'
    };

    // Create a mock product
    const createRes = await request.post('/api/catalog', {
      headers: authHeaders,
      data: {
        name: 'Caching Test Cake',
        price: '20.00',
        description: 'A delicious cake to test Edge caching',
        item_type: 'product'
      }
    });
    const catalogData = await createRes.json();
    const productId = catalogData.id;

    // Simulate builder has generated storefront layout via internal webhook
    await request.post('/api/storefront/webhook/invalidate', {
      data: { tags: [`tenant-id:maya_bakery`] }
    });

    // 2. Fetch Storefront Product Page
    let response = await request.get(`/api/storefront/maya_bakery/${productId}`);
    expect(response.ok()).toBeTruthy();

    // Validate Cache-Tag exists
    const headers = response.headers();
    expect(headers['cache-tag']).toContain(`storefront:product:maya_bakery:${productId}`);

    // Check initial inventory state
    let html = await response.text();
    expect(html).not.toContain('Sold Out');

    // 3. Owner updates inventory to 0 (Sold out)
    const posRes = await request.post('/api/inventory/commit', {
      headers: authHeaders,
      data: {
        product_id: productId,
        quantity: 9999, // deplete all stock
        lock_id: ''
      }
    });

    // Give PubSub a moment to trigger Edge purge
    await page.waitForTimeout(500);

    // 4. Fetch Storefront Product Page again, verify Cache Miss + Sold Out status
    response = await request.get(`/api/storefront/maya_bakery/${productId}`);
    expect(response.ok()).toBeTruthy();

    html = await response.text();
    // Due to the optimistic UI & edge caching, this string replacement happens inside `inject_dynamic_inventory`
    expect(html).toContain('Sold Out');
  });
});
