# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: pos-inventory-sync.spec.ts >> POS Inventory Sync - E2E Race Condition >> POS terminal applies lock and prevents double booking online
- Location: src/e2e/pos-inventory-sync.spec.ts:4:7

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: true
Received: false
```

# Test source

```ts
  1   | import { test, expect } from '@playwright/test';
  2   |
  3   | test.describe('POS Inventory Sync - E2E Race Condition', () => {
  4   |   test('POS terminal applies lock and prevents double booking online', async ({ page }) => {
  5   |     const tenantId = 'e2e-tenant-pos';
  6   |     const productId = 'e2e-product-cake-pos';
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
  23  |     if (!reserveRes.ok()) { console.log(await reserveRes.text()); }
> 24  |     expect(reserveRes.ok()).toBe(true);
      |                             ^ Error: expect(received).toBe(expected) // Object.is equality
  25  |     const lockData = await reserveRes.json();
  26  |     expect(lockData.success).toBe(true);
  27  |
  28  |     // Simulate Online User (User A) attempting checkout for the same item
  29  |     const reserveRes2 = await page.request.post('/api/v1/payments/terminal/reserve', {
  30  |         data: {
  31  |             tenant_id: tenantId,
  32  |             product_id: productId,
  33  |             quantity: 1,
  34  |             ttl_seconds: 15
  35  |         },
  36  |         headers: {
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
```