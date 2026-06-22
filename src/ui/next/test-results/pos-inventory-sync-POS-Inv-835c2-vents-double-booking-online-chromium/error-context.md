# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: pos-inventory-sync.spec.ts >> POS Inventory Sync - E2E Race Condition >> POS terminal applies lock and prevents double booking online
- Location: src/e2e/pos-inventory-sync.spec.ts:4:7

# Error details

```
Error: expect(received).toContain(expected) // indexOf

Expected substring: "another customer"
Received string:    "Backend connection failed"
```

# Test source

```ts
  1   | import { test, expect } from '@playwright/test';
  2   |
  3   | test.describe('POS Inventory Sync - E2E Race Condition', () => {
  4   |   test('POS terminal applies lock and prevents double booking online', async ({ page }) => {
  5   |     const tenantId = 'e2e-tenant-pos-1781845570';
  6   |     const productId = 'e2e-product-cake-pos-1781845570';
  7   |
  8   |     // Simulate POS (User B) acquiring lock
  9   |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  10  |         data: {
  11  |             tenant_id: tenantId,
  12  |             product_id: productId,
  13  |             quantity: 1,
  14  |             ttl_seconds: 15
  15  |         },
  16  |         headers: {
  17  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  18  |             'x-tenant-id': tenantId
  19  |         }
  20  |     });
  21  |
  22  |
  23  |     const lockData = await reserveRes.json();
  24  |
  25  |
  26  |     // Simulate Online User (User A) attempting checkout for the same item
  27  |     const reserveRes2 = await page.request.post('/api/v1/payments/terminal/reserve', {
  28  |         data: {
  29  |             tenant_id: tenantId,
  30  |             product_id: productId,
  31  |             quantity: 1,
  32  |             ttl_seconds: 15
  33  |         },
  34  |         headers: {
  35  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  36  |             'x-tenant-id': tenantId
  37  |         }
  38  |     });
  39  |
  40  |     // It should fail gracefully
  41  |     const lockData2 = await reserveRes2.json();
  42  |
> 43  |     expect(lockData2.error_message).toContain('another customer');
      |                                     ^ Error: expect(received).toContain(expected) // indexOf
  44  |
  45  |     // POS (User B) completes checkout
  46  |     const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
  47  |         data: {
  48  |             tenant_id: tenantId,
  49  |             product_id: productId,
  50  |             quantity: 1,
  51  |             lock_id: lockData.lock_id
  52  |         },
  53  |         headers: {
  54  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  55  |             'x-tenant-id': tenantId
  56  |         }
  57  |     });
  58  |
  59  |
  60  |   });
  61  |
  62  |   test('Online checkout UI shows Item just sold out when POS locks item', async ({ page }) => {
  63  |     const tenantId = 'e2e-tenant';
  64  |     const productId = 'e2e-product-cake';
  65  |
  66  |     // 1. Setup tenant info in local storage for checkout page
  67  |     await page.goto('/checkout');
  68  |     await page.evaluate((tenant) => {
  69  |       localStorage.setItem('tenant', tenant);
  70  |       localStorage.setItem('customer_id', 'e2e-customer');
  71  |     }, tenantId);
  72  |
  73  |     // Simulate POS (User B) acquiring lock
  74  |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  75  |         data: {
  76  |             tenant_id: tenantId,
  77  |             product_id: productId,
  78  |             quantity: 1,
  79  |             ttl_seconds: 15
  80  |         },
  81  |         headers: {
  82  |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  83  |             'x-tenant-id': tenantId
  84  |         }
  85  |     });
  86  |
  87  |
  88  |     const lockData = await reserveRes.json();
  89  |
  90  |
  91  |     // 2. Navigate to checkout page for the locked product
  92  |     await page.goto(`/checkout?product_id=${productId}&quantity=1`);
  93  |
  94  |     // 3. Click the Pay button
  95  |     await page.getByRole('button', { name: 'Pay' }).click();
  96  |
  97  |     // 4. Verify the "Item just sold out" message appears
  98  |     await expect(page.getByText('Item just sold out.')).toBeVisible();
  99  |
  100 |     // Cleanup: Release lock so it doesn't affect other tests if they run concurrently
  101 |     // (Actually the lock will expire in 15 seconds, but let's release it cleanly)
  102 |     await page.request.post('/api/v1/payments/terminal/commit', {
  103 |         data: {
  104 |             tenant_id: tenantId,
  105 |             product_id: productId,
  106 |             quantity: 1,
  107 |             lock_id: lockData.lock_id
  108 |         },
  109 |         headers: {
  110 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  111 |             'x-tenant-id': tenantId
  112 |         }
  113 |     });
  114 |   });
  115 |   test('Commit inventory correctly deducts stock', async ({ page }) => {
  116 |     const tenantId = 'e2e-tenant-pos-additional';
  117 |     const productId = 'e2e-product-cake-pos-additional';
  118 |
  119 |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  120 |         data: {
  121 |             tenant_id: tenantId,
  122 |             product_id: productId,
  123 |             quantity: 1,
  124 |             ttl_seconds: 15
  125 |         },
  126 |         headers: {
  127 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
  128 |             'x-tenant-id': tenantId
  129 |         }
  130 |     });
  131 |
  132 |     const lockData = await reserveRes.json();
  133 |
  134 |
  135 |     const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
  136 |         data: {
  137 |             tenant_id: tenantId,
  138 |             product_id: productId,
  139 |             quantity: 1,
  140 |             lock_id: lockData.lock_id
  141 |         },
  142 |         headers: {
  143 |             'x-spiffe-id': 'spiffe://ohc/org/' + tenantId + '/agent/browser',
```