# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: pos-inventory-sync.spec.ts >> POS Inventory Sync - E2E Race Condition >> Commit inventory correctly deducts stock
- Location: src/e2e/pos-inventory-sync.spec.ts:119:7

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: true
Received: false
```

# Test source

```ts
  37  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  38  |             'x-tenant-id': tenantId
  39  |         }
  40  |     });
  41  |
  42  |     // It should fail gracefully
  43  |     const lockData2 = await reserveRes2.json();
  44  |     expect(lockData2.success).toBe(false);
  45  |     expect(lockData2.error_message).toContain('another customer');
  46  |
  47  |     // POS (User B) completes checkout
  48  |     const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
  49  |         data: {
  50  |             tenant_id: tenantId,
  51  |             product_id: productId,
  52  |             quantity: 1,
  53  |             lock_id: lockData.lock_id
  54  |         },
  55  |         headers: {
  56  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  57  |             'x-tenant-id': tenantId
  58  |         }
  59  |     });
  60  |
  61  |     expect(commitRes.ok()).toBe(true);
  62  |   });
  63  |
  64  |   test('Online checkout UI shows Item just sold out when POS locks item', async ({ page }) => {
  65  |     const tenantId = 'e2e-tenant';
  66  |     const productId = 'e2e-product-cake';
  67  |
  68  |     // 1. Setup tenant info in local storage for checkout page
  69  |     await page.goto('/checkout');
  70  |     await page.evaluate((tenant) => {
  71  |       localStorage.setItem('tenant', tenant);
  72  |       localStorage.setItem('customer_id', 'e2e-customer');
  73  |     }, tenantId);
  74  |
  75  |     // Simulate POS (User B) acquiring lock
  76  |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  77  |         data: {
  78  |             tenant_id: tenantId,
  79  |             product_id: productId,
  80  |             quantity: 1,
  81  |             ttl_seconds: 15
  82  |         },
  83  |         headers: {
  84  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  85  |             'x-tenant-id': tenantId
  86  |         }
  87  |     });
  88  |
  89  |
  90  |     if (!reserveRes.ok()) { console.log(await reserveRes.text()); }
  91  |     expect(reserveRes.ok()).toBe(true);
  92  |     const lockData = await reserveRes.json();
  93  |     expect(lockData.success).toBe(true);
  94  |
  95  |     // 2. Navigate to checkout page for the locked product
  96  |     await page.goto(`/checkout?product_id=${productId}&quantity=1`);
  97  |
  98  |     // 3. Click the Pay button
  99  |     await page.getByRole('button', { name: 'Pay' }).click();
  100 |
  101 |     // 4. Verify the "Item just sold out" message appears
  102 |     await expect(page.locator('h3', { hasText: 'Oops! Item just sold out.' })).toBeVisible();
  103 |
  104 |     // Cleanup: Release lock so it doesn't affect other tests if they run concurrently
  105 |     // (Actually the lock will expire in 15 seconds, but let's release it cleanly)
  106 |     await page.request.post('/api/v1/payments/terminal/commit', {
  107 |         data: {
  108 |             tenant_id: tenantId,
  109 |             product_id: productId,
  110 |             quantity: 1,
  111 |             lock_id: lockData.lock_id
  112 |         },
  113 |         headers: {
  114 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  115 |             'x-tenant-id': tenantId
  116 |         }
  117 |     });
  118 |   });
  119 |   test('Commit inventory correctly deducts stock', async ({ page }) => {
  120 |     const tenantId = 'e2e-tenant-pos-additional';
  121 |     const productId = 'e2e-product-cake-pos-additional';
  122 |
  123 |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  124 |         data: {
  125 |             tenant_id: tenantId,
  126 |             product_id: productId,
  127 |             quantity: 1,
  128 |             ttl_seconds: 15
  129 |         },
  130 |         headers: {
  131 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  132 |             'x-tenant-id': tenantId
  133 |         }
  134 |     });
  135 |
  136 |     if (!reserveRes.ok()) { console.log(await reserveRes.text()); }
> 137 |     expect(reserveRes.ok()).toBe(true);
      |                             ^ Error: expect(received).toBe(expected) // Object.is equality
  138 |     const lockData = await reserveRes.json();
  139 |     expect(lockData.success).toBe(true);
  140 |
  141 |     const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
  142 |         data: {
  143 |             tenant_id: tenantId,
  144 |             product_id: productId,
  145 |             quantity: 1,
  146 |             lock_id: lockData.lock_id
  147 |         },
  148 |         headers: {
  149 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  150 |             'x-tenant-id': tenantId
  151 |         }
  152 |     });
  153 |
  154 |
  155 |     const commitData = await commitRes.json();
  156 |     expect(commitData.success).toBe(true);
  157 |   });
  158 |
  159 |   test('Operations Agent generates a Restock notification in the owner feed when item sells out', async ({ page }) => {
  160 |     // 1. Log in to get token
  161 |     await page.goto('/login');
  162 |     await page.getByPlaceholder('Email address').fill('admin@ohc.local');
  163 |     await page.getByPlaceholder('Password').fill('admin');
  164 |     await page.getByRole('button', { name: 'Sign In' }).click();
  165 |     await expect(page.locator('text=Dashboard').first()).toBeVisible({ timeout: 15000 });
  166 |
  167 |     const response = await page.request.post('/api/v1/auth/login', {
  168 |         data: {
  169 |             email: 'admin@ohc.local',
  170 |             password: 'admin'
  171 |         }
  172 |     });
  173 |     expect(response.ok()).toBeTruthy();
  174 |     const { token } = await response.json();
  175 |
  176 |     const tenantId = 'default';
  177 |     const productId = 'e2e-product-restock-' + Date.now();
  178 |
  179 |     // 2. Create the product with stock 1
  180 |     const createProductRes = await page.request.post('/api/v1/catalog/products', {
  181 |         headers: { Authorization: `Bearer ${token}` },
  182 |         data: {
  183 |             id: productId,
  184 |             title: 'Limited Restock Item',
  185 |             inventory_count: 1,
  186 |             price_cents: 1000
  187 |         }
  188 |     });
  189 |     expect(createProductRes.ok()).toBeTruthy();
  190 |
  191 |     // Simulate POS (User B) acquiring lock
  192 |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  193 |         data: {
  194 |             tenant_id: tenantId,
  195 |             product_id: productId,
  196 |             quantity: 1,
  197 |             ttl_seconds: 15
  198 |         },
  199 |         headers: {
  200 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  201 |             'x-tenant-id': tenantId
  202 |         }
  203 |     });
  204 |
  205 |     if (!reserveRes.ok()) { console.log(await reserveRes.text()); }
  206 |     expect(reserveRes.ok()).toBe(true);
  207 |     const lockData = await reserveRes.json();
  208 |     expect(lockData.success).toBe(true);
  209 |
  210 |     // POS (User B) completes checkout
  211 |     const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
  212 |         data: {
  213 |             tenant_id: tenantId,
  214 |             product_id: productId,
  215 |             quantity: 1,
  216 |             lock_id: lockData.lock_id
  217 |         },
  218 |         headers: {
  219 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  220 |             'x-tenant-id': tenantId
  221 |         }
  222 |     });
  223 |
  224 |     expect(commitRes.ok()).toBe(true);
  225 |
  226 |     await page.waitForTimeout(5000);
  227 |
  228 |     // Navigate to Action Center
  229 |     await page.goto('/dashboard');
  230 |
  231 |     // Check if the agent action request appears in the feed
  232 |     await expect(page.locator('body')).toContainText('Action Request: Reorder', { timeout: 30000 });
  233 |   });
  234 |
  235 | });
  236 |
```