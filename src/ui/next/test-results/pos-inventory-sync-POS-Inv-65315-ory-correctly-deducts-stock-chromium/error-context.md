# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: pos-inventory-sync.spec.ts >> POS Inventory Sync - E2E Race Condition >> Commit inventory correctly deducts stock
- Location: src/e2e/pos-inventory-sync.spec.ts:117:7

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: true
Received: false
```

# Test source

```ts
  34  |         },
  35  |         headers: {
  36  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  37  |             'x-tenant-id': tenantId
  38  |         }
  39  |     });
  40  |
  41  |     // It should fail gracefully
  42  |     const lockData2 = await reserveRes2.json();
  43  |     expect(lockData2.success).toBe(false);
  44  |     expect(lockData2.error_message).toContain('another customer');
  45  |
  46  |     // POS (User B) completes checkout
  47  |     const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
  48  |         data: {
  49  |             tenant_id: tenantId,
  50  |             product_id: productId,
  51  |             quantity: 1,
  52  |             lock_id: lockData.lock_id
  53  |         },
  54  |         headers: {
  55  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  56  |             'x-tenant-id': tenantId
  57  |         }
  58  |     });
  59  |
  60  |     expect(commitRes.ok()).toBe(true);
  61  |   });
  62  |
  63  |   test('Online checkout UI shows Item just sold out when POS locks item', async ({ page }) => {
  64  |     const tenantId = 'e2e-tenant';
  65  |     const productId = 'e2e-product-cake';
  66  |
  67  |     // 1. Setup tenant info in local storage for checkout page
  68  |     await page.goto('/checkout');
  69  |     await page.evaluate((tenant) => {
  70  |       localStorage.setItem('tenant', tenant);
  71  |       localStorage.setItem('customer_id', 'e2e-customer');
  72  |     }, tenantId);
  73  |
  74  |     // Simulate POS (User B) acquiring lock
  75  |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  76  |         data: {
  77  |             tenant_id: tenantId,
  78  |             product_id: productId,
  79  |             quantity: 1,
  80  |             ttl_seconds: 15
  81  |         },
  82  |         headers: {
  83  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  84  |             'x-tenant-id': tenantId
  85  |         }
  86  |     });
  87  |
  88  |
  89  |     expect(reserveRes.ok()).toBe(true);
  90  |     const lockData = await reserveRes.json();
  91  |     expect(lockData.success).toBe(true);
  92  |
  93  |     // 2. Navigate to checkout page for the locked product
  94  |     await page.goto(`/checkout?product_id=${productId}&quantity=1`);
  95  |
  96  |     // 3. Click the Pay button
  97  |     await page.getByRole('button', { name: 'Pay' }).click();
  98  |
  99  |     // 4. Verify the "Item just sold out" message appears
  100 |     await expect(page.locator('h3', { hasText: 'Oops! Item just sold out.' })).toBeVisible();
  101 |
  102 |     // Cleanup: Release lock so it doesn't affect other tests if they run concurrently
  103 |     // (Actually the lock will expire in 15 seconds, but let's release it cleanly)
  104 |     await page.request.post('/api/v1/payments/terminal/commit', {
  105 |         data: {
  106 |             tenant_id: tenantId,
  107 |             product_id: productId,
  108 |             quantity: 1,
  109 |             lock_id: lockData.lock_id
  110 |         },
  111 |         headers: {
  112 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  113 |             'x-tenant-id': tenantId
  114 |         }
  115 |     });
  116 |   });
  117 |   test('Commit inventory correctly deducts stock', async ({ page }) => {
  118 |     const tenantId = 'e2e-tenant-pos-additional';
  119 |     const productId = 'e2e-product-cake-pos-additional';
  120 |
  121 |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  122 |         data: {
  123 |             tenant_id: tenantId,
  124 |             product_id: productId,
  125 |             quantity: 1,
  126 |             ttl_seconds: 15
  127 |         },
  128 |         headers: {
  129 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  130 |             'x-tenant-id': tenantId
  131 |         }
  132 |     });
  133 |
> 134 |     expect(reserveRes.ok()).toBe(true);
      |                             ^ Error: expect(received).toBe(expected) // Object.is equality
  135 |     const lockData = await reserveRes.json();
  136 |     expect(lockData.success).toBe(true);
  137 |
  138 |     const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
  139 |         data: {
  140 |             tenant_id: tenantId,
  141 |             product_id: productId,
  142 |             quantity: 1,
  143 |             lock_id: lockData.lock_id
  144 |         },
  145 |         headers: {
  146 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  147 |             'x-tenant-id': tenantId
  148 |         }
  149 |     });
  150 |
  151 |
  152 |     const commitData = await commitRes.json();
  153 |     expect(commitData.success).toBe(true);
  154 |   });
  155 |
  156 | });
  157 |
```