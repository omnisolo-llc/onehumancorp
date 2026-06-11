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
  1  | import { test, expect } from '@playwright/test';
  2  |
  3  | test.describe('POS Inventory Sync - E2E Race Condition', () => {
  4  |   test('POS terminal applies lock and prevents double booking online', async ({ page }) => {
  5  |     const tenantId = 'e2e-tenant-pos';
  6  |     const productId = 'prod_123';
  7  |
  8  |     // Simulate POS (User B) acquiring lock
  9  |     const reserveRes = await page.request.post('/api/v1/payments/terminal/reserve', {
  10 |         data: {
  11 |             tenant_id: tenantId,
  12 |             product_id: productId,
  13 |             quantity: 1,
  14 |             ttl_seconds: 15
  15 |         },
  16 |         headers: {
  17 |             'x-tenant-id': tenantId
  18 |         }
  19 |     });
  20 |
> 21 |     expect(reserveRes.ok()).toBe(true);
     |                             ^ Error: expect(received).toBe(expected) // Object.is equality
  22 |     const lockData = await reserveRes.json();
  23 |     expect(lockData.success).toBe(true);
  24 |
  25 |     // Simulate Online User (User A) attempting checkout for the same item
  26 |     const reserveRes2 = await page.request.post('/api/v1/payments/terminal/reserve', {
  27 |         data: {
  28 |             tenant_id: tenantId,
  29 |             product_id: productId,
  30 |             quantity: 1,
  31 |             ttl_seconds: 15
  32 |         },
  33 |         headers: {
  34 |             'x-tenant-id': tenantId
  35 |         }
  36 |     });
  37 |
  38 |     // It should fail gracefully
  39 |     const lockData2 = await reserveRes2.json();
  40 |     expect(lockData2.success).toBe(false);
  41 |     expect(lockData2.error_message).toContain('another customer');
  42 |
  43 |     // POS (User B) completes checkout
  44 |     const commitRes = await page.request.post('/api/v1/payments/terminal/commit', {
  45 |         data: {
  46 |             tenant_id: tenantId,
  47 |             product_id: productId,
  48 |             quantity: 1,
  49 |             lock_id: lockData.lock_id
  50 |         },
  51 |         headers: {
  52 |             'x-tenant-id': tenantId
  53 |         }
  54 |     });
  55 |
  56 |     expect(commitRes.ok()).toBe(true);
  57 |   });
  58 | });
  59 |
```